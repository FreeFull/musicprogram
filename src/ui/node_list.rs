use tuix::*;

use super::model::*;

#[derive(Default)]
pub struct NodeList {
    list: Entity,
}

impl Widget for NodeList {
    type Ret = Entity;

    type Data = NodeData;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.list =
            widgets::ScrollContainer::new().build(state, entity, |builder| builder.class("list"));
        entity
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        todo!()
    }
}
