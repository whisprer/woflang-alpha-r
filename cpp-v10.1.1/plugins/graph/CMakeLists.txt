# plugins/graph/CMakeLists.txt

woflang_add_plugin(graph_theory_ops
    graph_theory_ops.cpp
)

woflang_add_plugin(graph_search_ops
    graph_search_ops.cpp
)

woflang_add_plugin(graph_coloring_ops
    graph_coloring_ops.cpp
)

woflang_add_plugin(graph_shortest_path_ops
    graph_shortest_path_ops.cpp
)