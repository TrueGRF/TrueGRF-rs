use super::{Action, ActionTrait, Output, vec_list, write_special};


pub struct ActionFName<'a> {
    pub probability: u8,
    pub name: &'a String,
}

pub struct ActionFId {
    pub probability: u8,
    pub id: u8,
}

pub enum ActionFNameOrId<'a> {
    Name(ActionFName<'a>),
    Id(ActionFId),
}

impl<'a> From<ActionFName<'a>> for ActionFNameOrId<'a> {
    fn from(item: ActionFName<'a>) -> Self {
        ActionFNameOrId::Name(item)
    }
}
impl From<ActionFId> for ActionFNameOrId<'_> {
    fn from(item: ActionFId) -> Self {
        ActionFNameOrId::Id(item)
    }
}

pub struct ActionFPart<'a> {
    pub firstbit: u8,
    pub bitcount: u8,

    pub names: Vec<ActionFNameOrId<'a>>
}

pub enum ActionF<'a> {
    Style { id: u8, name: Option<&'a String>, parts: &'a Vec<ActionFPart<'a>> },
}

impl<'a> ActionTrait for ActionF<'a> {
    fn write(&self, output: &mut Output) {
        match self {
            ActionF::Style { id, name, parts } => {
                let mut name_data = Vec::new();
                let id = match *name {
                    Some(name) => {
                        name_data.extend([0x7f]);
                        name_data.extend(&*name.as_bytes());
                        name_data.extend([0x00]);
                        name_data.extend([0x00]);
                        *id | 0x80
                    },
                    None => {
                        *id
                    }
                };

                let mut data = Vec::new();
                for part in *parts {
                    data.extend([
                        part.names.len() as u8,
                        part.firstbit,
                        part.bitcount,
                    ]);

                    for name in &part.names {
                        match name {
                            ActionFNameOrId::Name(item) => {
                                data.extend([item.probability]);
                                data.extend(&*item.name.as_bytes());
                                data.extend([0x00]);
                            },
                            ActionFNameOrId::Id(item) => {
                                data.extend([item.probability | 0x80]);
                                data.extend([item.id]);
                            },
                        }
                    }
                }

                write_special(output, Action::ActionF, &vec_list!(
                    [id],
                    name_data,
                    [parts.len() as u8],
                    data
                ));
            }
        }
    }
}
