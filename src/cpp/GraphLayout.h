#ifndef GRAPHLAYOUT_H
#define GRAPHLAYOUT_H

#include <unordered_map>
#include <vector>

#include "Point2f.h"

class GraphLayout
{
public:
    struct GraphEdge
    {
        uint64_t target;
        std::vector<Point2f> polyline;
        enum ArrowDirection { Down, Left, Up, Right, None };
        ArrowDirection arrow = ArrowDirection::Down;

        explicit GraphEdge(uint64_t target) : target(target) {}
    };

    struct GraphBlock
    {
        int x = 0;
        int y = 0;
        int width = 0;
        int height = 0;
        // This is a unique identifier, e.g. offset in the case of rizin blocks
        uint64_t entry;
        // Edges
        std::vector<GraphEdge> edges;
    };
    using Graph = std::unordered_map<uint64_t, GraphBlock>;

    struct LayoutConfig
    {
        int blockVerticalSpacing = 40;
        int blockHorizontalSpacing = 20;
        int edgeVerticalSpacing = 10;
        int edgeHorizontalSpacing = 10;
    };

    GraphLayout(const LayoutConfig &layout_config) : layoutConfig(layout_config) {}
    virtual ~GraphLayout() {}
    virtual void CalculateLayout(Graph &blocks, uint64_t entry, int &width, int &height) const = 0;
    virtual void setLayoutConfig(const LayoutConfig &config) { this->layoutConfig = config; };

protected:
    LayoutConfig layoutConfig;
};

#endif // GRAPHLAYOUT_H
