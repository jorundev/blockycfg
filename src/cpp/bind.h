#include <stdint.h>

typedef struct GraphContext {
	void	*layout;
	void	*graph;
}	GraphContext;

typedef struct EdgeEntry {
	uint64_t	target;
}	EdgeEntry;

typedef struct BlockEntry {
	uint64_t	address;
	int32_t		width;
	int32_t		height;
	EdgeEntry	*edges;
	uint32_t	edge_count;
}	BlockEntry;

typedef struct Point {
	float x, y;
}	Point;

typedef struct EdgeData {
	Point		*polyline;
	uint32_t	point_count;
}	EdgeData;

typedef struct BlockData {
	uint64_t	address;
	int32_t		x;
	int32_t		y;
	int32_t		width;
	int32_t		height;
	EdgeData	*edges;
	uint32_t	edge_count;
}	BlockData;

typedef struct Layout {
	BlockData	*blocks;
	uint32_t	block_count;
	int32_t		width;
	int32_t		height;
}	Layout;

#ifdef __cplusplus
extern "C" {
#endif

GraphContext *create_context();
void delete_context(GraphContext *context);
void add_block(GraphContext *ctx, const BlockEntry *entry);
Layout *calculate_layout(GraphContext *ctx, uint64_t entry_point);
void delete_layout(Layout *layout);

#ifdef __cplusplus
}
#endif
