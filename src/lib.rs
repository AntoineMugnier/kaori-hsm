struct NoneState;

pub trait State<T: StateMachine>{
    fn init(&self, sm : &mut T);

    fn entry(&self, sm: &mut T);

    fn exit(&self, sm : &mut T);

    fn handle(&self, sm : &mut T, evt: T::Evt);

}

pub trait StateMachine{
    type Evt;
    fn dispatch(&mut self, evt : Self::Evt){}
    fn get_curr_state(&mut self) -> Box<dyn State<Self>>;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
