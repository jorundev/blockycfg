#include "bind.h"
#include "GraphGridLayout.h"
#include "GraphLayout.h"

extern "C" GraphContext *create_context()
{
	GraphContext *ctx = new GraphContext;

	ctx->layout = new GraphGridLayout(GraphGridLayout::LayoutType::Narrow);
	ctx->graph = new GraphLayout::Graph;

	return ctx;
}

extern "C" void delete_context(GraphContext *ctx)
{
	delete (GraphGridLayout *)ctx->layout;
	delete (GraphLayout::Graph *)ctx->graph;
	delete ctx;
}

extern "C" void add_block(GraphContext *ctx, const BlockEntry *entry)
{
	GraphLayout::Graph *gph = (GraphLayout::Graph *)ctx->graph;

	GraphLayout::GraphBlock block;

	block.entry = entry->address;
	block.width = entry->width;
	block.height = entry->height;
	block.edges.reserve(entry->edge_count);
	for (uint32_t i = 0; i < entry->edge_count; ++i) {
		block.edges.push_back(GraphGridLayout::GraphEdge(entry->edges[i].target));
	}

	(*gph)[block.entry] = block;
}

extern "C" Layout *calculate_layout(GraphContext *ctx, uint64_t entry_point)
{
	Layout *ret = new Layout;

	GraphGridLayout::Graph *gph = (GraphLayout::Graph *)ctx->graph;
	GraphGridLayout	*lt = (GraphGridLayout *)ctx->layout;

	lt->CalculateLayout(*gph, entry_point, (int&)ret->width, (int&)ret->height);

	ret->block_count = gph->size();
	ret->blocks = new BlockData[ret->block_count];

	uint32_t i = 0;
	for (auto& block : *gph) {
		ret->blocks[i].address = block.second.entry;
		ret->blocks[i].edge_count = block.second.edges.size();
		ret->blocks[i].edges = new EdgeData[block.second.edges.size()];
		ret->blocks[i].x = block.second.x;
		ret->blocks[i].y = block.second.y;
		ret->blocks[i].width = block.second.width;
		ret->blocks[i].height = block.second.height;
		uint32_t o = 0;
		for (auto& edge : block.second.edges) {
			ret->blocks[i].edges[o].point_count = edge.polyline.size();
			ret->blocks[i].edges[o].polyline = (Point *)&edge.polyline[0];
			++o;
		}
		++i;
	}
	return ret;
}

extern "C" void delete_layout(Layout *layout)
{
	for (uint32_t i = 0; i < layout->block_count; ++i) {
		delete[] layout->blocks[i].edges;
	}
	delete[] layout->blocks;
	delete layout;
}
