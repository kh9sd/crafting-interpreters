#!/home/keab/.pyenv/shims/python

class TODO_AST_Node:
    def __init__(self, name: str, fields: list[tuple[str, str]]) -> None:
        self.name = name
        self.fields = fields
    
    def print_fields_as_pub(self) -> str:
        return "\n".join(f"pub {name}: {type}," for type, name in self.fields)


def print_ast(base_name: str, subclasses: list[TODO_AST_Node]):
    print(f"pub trait {base_name} {{}}")

    for subclass in subclasses:
        print(f"""
pub struct {subclass.name} {{
    {subclass.print_fields_as_pub()}
}}

impl {base_name} for {subclass.name} {{}}
""")
    

if __name__ == "__main__":
    print_ast("Expr", 
              [TODO_AST_Node("Binary", 
                             [["Box<dyn Expr>", "left"], 
                              ["Token", "operator"], 
                              ["Box<dyn Expr>", "right"]]),
                TODO_AST_Node("Grouping", 
                             [["Box<dyn Expr>", "expression"]]),
                TODO_AST_Node("Unary", 
                             [["Token", "operator"], 
                              ["Box<dyn Expr>", "right"]]),
                            
                             ])
