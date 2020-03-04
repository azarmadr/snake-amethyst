use std::marker::PhantomData;

use amethyst::{
    core::{ArcThreadPool, SystemBundle, SystemDesc},
    ecs::prelude::{Dispatcher, DispatcherBuilder, System, World, WorldExt},
    error::Error,
    DataDispose, DataInit,
};


pub struct GameExecData<'a, 'b> {
    core_dispatcher: Option<Dispatcher<'a, 'b>>,
    running_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> GameExecData<'a, 'b> {
    /// Update game data
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            if let Some(dispatcher) = self.running_dispatcher.as_mut() {
                dispatcher.dispatch(&world);
            }
        }
        if let Some(dispatcher) = self.core_dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }
    }
}

impl<'a,'b> DataDispose for GameExecData<'a,'b> {
    fn dispose(&mut self, world: &mut World) {
        if let Some(core_disp) = self.core_dispatcher.take() {
            core_disp.dispose(world);
        }

        if let Some(running_disp) = self.running_dispatcher.take() {
            running_disp.dispose(world);
        }
    }
}



pub struct GameExecBuilder<'a, 'b> {
    base_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    running_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

impl<'a, 'b> Default for GameExecBuilder<'a, 'b> {
    fn default() -> Self {
        GameExecBuilder::new()
    }
}

impl<'a, 'b> GameExecBuilder<'a, 'b> {
    pub fn new() -> Self {
        GameExecBuilder {
            base_dispatcher_operations: vec![],
            running_dispatcher_operations: vec![],
        }
    }

    pub fn _with_base<SD, S>(
        mut self,
        system_desc: SD,
        name: &'static str,
        dependencies: &'static [&'static str],
    ) -> Self
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + 'static + Send,
    {
        let dispatcher_operation = Box::new(AddSystem {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.base_dispatcher_operations.push(dispatcher_operation);
        self
    }

    pub fn with_base_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.base_dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
        self
    }

    pub fn with_running<SD, S>(
        mut self,
        system_desc: SD,
        name: &'static str,
        dependencies: &'static [&'static str],
    ) -> Self
    where
        SD: SystemDesc<'a, 'b, S> + 'static,
        S: for<'c> System<'c> + 'static + Send,
    {
        let dispatcher_operation = Box::new(AddSystem {
            system_desc,
            name,
            dependencies,
            marker: PhantomData::<S>,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;
        self.running_dispatcher_operations
            .push(dispatcher_operation);
        self
    }
}

impl<'a, 'b> DataInit<GameExecData<'a, 'b>> for GameExecBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> GameExecData<'a, 'b> {
        let core_dispatcher = Some(build_dispatcher(world, self.base_dispatcher_operations));
        let running_dispatcher = Some(build_dispatcher(world, self.running_dispatcher_operations));

        GameExecData {
            core_dispatcher,
            running_dispatcher,
        }
    }
}

fn build_dispatcher<'a, 'b>(
    world: &mut World,
    dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
) -> Dispatcher<'a, 'b> {
    let mut dispatcher_builder = DispatcherBuilder::new();

    #[cfg(not(no_threading))]
    {
        let pool = world.read_resource::<ArcThreadPool>().clone();
        dispatcher_builder = dispatcher_builder.with_pool((*pool).clone());
    }

    dispatcher_operations
        .into_iter()
        .try_for_each(|dispatcher_operation| {
            dispatcher_operation.exec(world, &mut dispatcher_builder)
        })
        .unwrap_or_else(|e| panic!("Failed to set up dispatcher: {}", e));

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(world);
    dispatcher
}

/// Trait to capture deferred dispatcher builder operations.
trait DispatcherOperation<'a, 'b> {
    /// Executes the dispatcher builder instruction.
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error>;
}

struct AddSystem<SD, S> {
    system_desc: SD,
    name: &'static str,
    dependencies: &'static [&'static str],
    marker: PhantomData<S>,
}

impl<'a, 'b, SD, S> DispatcherOperation<'a, 'b> for AddSystem<SD, S>
where
    SD: SystemDesc<'a, 'b, S>,
    S: for<'s> System<'s> + Send + 'a,
{
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let system = self.system_desc.build(world);
        dispatcher_builder.add(system, self.name, self.dependencies);
        Ok(())
    }
}

struct AddBundle<B> {
    bundle: B,
}

impl<'a, 'b, B> DispatcherOperation<'a, 'b> for AddBundle<B>
where
    B: SystemBundle<'a, 'b>,
{
    fn exec(
        self: Box<Self>,
        world: &mut World,
        dispatcher_builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        self.bundle.build(world, dispatcher_builder)?;
        Ok(())
    }
}