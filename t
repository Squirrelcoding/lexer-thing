Env { 
    vals: {
        "makeCounter": 
        Func(
            Func { 
                instructions: Block(
                    [
                        Declaration(
                            Declaration { 
                                ident: "i", 
                                val: Num(0) 
                            }
                        ), 
                        Declaration(
                            Declaration { 
                                ident: "count", 
                                val: Func(
                                    Func { 
                                        instructions: Block(
                                            [
                                                Assignment(
                                                    Declaration { 
                                                        ident: "i", 
                                                        val: Bin(
                                                            BinExpr { 
                                                                lhs: Var("i"), 
                                                                rhs: Num(1), 
                                                                op: Add 
                                                            }
                                                        ) 
                                                    }
                                                ), 
                                                Print(Var("i"))
                                            ]
                                        ), 
                                        args: [], 
                                        closure: Env { 
                                            vals: {}, 
                                            parent: None 
                                        } 
                                    }
                                ) 
                            }
                        ), 
                        Return(Var("count"))
                    ]
                ), 
                args: [], 
                closure: Env { 
                    vals: {}, 
                    parent: None 
                } 
            }
        ), 
        "count": Func(
            Func { 
                instructions: Block(
                    [
                        Assignment(
                            Declaration { 
                                ident: "i", 
                                val: Bin(
                                    BinExpr { 
                                        lhs: Var("i"), 
                                        rhs: Num(1), 
                                        op: Add 
                                    }
                                ) 
                            }
                        ), 
                        Print(Var("i"))
                    ]
                ), 
                args: [], 
                closure: Env { 
                    vals: {}, 
                    parent: None 
                }
            }
        )
    }, 
    parent: None 
}