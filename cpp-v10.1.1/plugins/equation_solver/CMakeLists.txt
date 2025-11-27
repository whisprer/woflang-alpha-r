# plugins/equation_solver/CMakeLists.txt

woflang_add_plugin(symbolic_rules_simplify_ops
    symbolic_rules_simplify_ops.cpp
)

woflang_add_plugin(symbolic_linear_solve_ops
    symbolic_linear_solve_ops.cpp
)

woflang_add_plugin(symbolic_pattern_solve_ops
    symbolic_pattern_solve_ops.cpp
)

woflang_add_plugin(symbolic_calc_ops
    symbolic_calc_ops.cpp
)

woflang_add_plugin(symbolic_logic_ops
    symbolic_logic_ops.cpp
)

woflang_add_plugin(symbolic_solver_ops
    symbolic_solver_ops.cpp
)