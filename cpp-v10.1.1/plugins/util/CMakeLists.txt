# plugins/utils/CMakeLists.txt

woflang_add_plugin(assert_ops
    assert_ops.cpp
)

woflang_add_plugin(io_debugs_ops
    io_debug_ops.cpp
)

woflang_add_plugin(repl_history_ops
    repl_history_ops.cpp
)

woflang_add_plugin(repl_suggest_ops
    repl_suggest_ops.cpp
)

woflang_add_plugin(stack_ops
    stack_ops.cpp
)
