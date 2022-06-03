use super::actions::{
    Action2RPN,
    ActionTrait,
    Feature,
    VarAction2,
    VarAction2Operator,
    VarAction2Switch,
    Variable,
};

use super::Output;

pub fn write_rpn_chain(output: &mut Output, cb: u8, chain: &Action2RPN::Chain, has_callback: bool) {
    /* Some chains have a callback as return value (instead of a value). These
     * callbacks are already defined before we enter this function on "cb". As
     * such, we cannot use "cb" for other chains, as it will overwrite the
     * callback chain. This in fact is just a bit of lazy programming, as what
     * we should be doing is defining the callback chain just before we write
     * the last chain. But this is for now far easier to program. See for
     * example the Action2::Industry callback. */
    let cb_child = cb + if has_callback { 1 } else { 0 };

    for (i, child) in chain.children.iter().enumerate() {
        write_rpn_chain(output, cb_child + i as u8, child, false);
    }

    /* Rewrite the switch with the correct setids in the result entries. */
    let mut switch = Vec::new();
    for case in &chain.switch {
        /* Use "cb" instead of "cb_child", as switches already take care of correcting for the + 1 by the caller. */
        switch.push(VarAction2Switch { result: case.result + cb as u16, left: case.left, right: case.right });
    }

    /* Rewrite the body to point to correct setids. */
    let mut body = Vec::new();
    for operator in &chain.body {
        let result = match operator {
            VarAction2Operator::Head(Variable::Variable { variable: 0x7e, parameter, shift: _, mask: _ }) => {
                /* Procedure entries refer to local setids; fix them to use the correct setid. */
                VarAction2Operator::Head(Variable::Global::Procedure(parameter.unwrap() + cb_child).into())
            },
            _ => operator.clone(),
        };
        body.push(result);
    }

    VarAction2::Advanced {
        set_id: cb,
        feature: Feature::Industries,
        related_object: false,
        variable: &body,
        switch: &switch,
        default: cb as u16
    }.write(output);
}
