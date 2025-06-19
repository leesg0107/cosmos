# 🌌 Cosmos - Multi-Dimensional Cake Graph

A revolutionary multi-dimensional graph visualization tool that represents complex relationships through interactive 3D cake structures.

## 🎂 What is Cosmos?

Cosmos transforms traditional flat graphs into intuitive **multi-dimensional cake structures**, where each "cake" represents a domain or project, and layers within each cake represent hierarchical relationships. Think of it as a 3D knowledge management system where you can:

- Create multiple independent cake structures
- Connect nodes within and across different cakes  
- Navigate through different conceptual layers
- Visualize complex multi-dimensional relationships

## ✨ Current Features

### 🎯 **Core Functionality**
- ✅ **Multi-Cake Architecture**: Create unlimited independent cake structures
- ✅ **Dynamic Layer System**: Automatically expanding layers (1→2→3→4+)
- ✅ **Cross-Cake Connections**: Inter-dimensional relationships between different cakes
- ✅ **Real-time Node Creation**: Instant node generation with type-specific layers

### 🎮 **Interactive Controls**
- ✅ **MacBook Navigation**: Two-finger scroll for panning, scroll wheel for zoom
- ✅ **Drag-to-Connect**: Intuitive node-to-node connection creation
- ✅ **Visual Feedback**: Real-time connection preview while dragging
- ✅ **Precision Editing**: No rotation interference - stable editing experience

### 🎨 **Visual Design**
- ✅ **4 Color Themes**: Default, Warm, Cool, Nature
- ✅ **Layer-Based Hierarchy**: 
  - 🌌 Root (Layer 1) - Gold
  - 💭 Concept (Layer 2) - Blue  
  - 📋 Task (Layer 3) - Pink
  - 📝 Note (Layer 4) - Green
- ✅ **Connection Types**: 
  - White lines for intra-cake connections
  - Red lines for inter-cake connections

### 🛠️ **User Interface**
- ✅ **Quick Creation**: Instant cake generation with "New Cake" button
- ✅ **Status Display**: Real-time cake/node count and layer information
- ✅ **Context Menus**: Right-click node creation
- ✅ **Edit Modes**: Double-click editing, node deletion

## 🚧 Development Status & Roadmap

### 🔧 **Areas Needing Improvement**

#### **Graphics Rendering Issues**
- **GPU Compatibility**: Occasional Metal renderer conflicts on macOS
- **Performance Optimization**: Large graphs can cause frame drops
- **Anti-aliasing**: Edge smoothing needs refinement
- **Visual Polish**: Node icons and text scaling could be improved

#### **Sand-Graph Data Structure Limitations**
- **Persistence**: No save/load functionality yet
- **Data Validation**: Limited constraint checking on connections
- **Memory Management**: Large graphs not optimized for memory usage
- **Serialization**: No export/import capabilities

#### **Advanced Features Missing**
- **Node Grouping**: Ability to create sub-groups within layers
- **Advanced Filtering**: Search and filter nodes by type/content
- **Layout Algorithms**: Auto-arrangement of nodes in layers
- **Animation**: Smooth transitions between states
- **Collaborative Editing**: Multi-user support

### 🎯 **Next Priorities**
1. **Stabilize Graphics Pipeline** - Fix Metal renderer issues
2. **Implement Save/Load** - Basic persistence functionality
3. **Performance Optimization** - Handle 1000+ node graphs smoothly
4. **Enhanced UI/UX** - Better visual feedback and interactions

## 🏗️ **Technical Architecture**

### **Core Components**
- **CakeStructure**: Multi-dimensional container with dynamic layer expansion
- **CakeNode**: Individual nodes with type-specific properties
- **Viewport**: Pan/zoom navigation system
- **InteractionMode**: State machine for different user interactions

### **Built With**
- **Rust** - Core application logic
- **egui** - Immediate mode GUI framework
- **eframe** - Application framework
- **nalgebra/glam** - 3D math operations

## 🚀 **Getting Started**

### **Prerequisites**
- Rust 1.70+ 
- macOS (primary development platform)

### **Installation**
```bash
git clone https://github.com/your-username/cosmos.git
cd cosmos
cargo run
```

### **Basic Usage**
1. **Create Cakes**: Click "🎂 New Cake" for instant creation
2. **Add Nodes**: Use layer-specific buttons (Layer 1-4) or right-click menu
3. **Connect Nodes**: Drag from one node to another
4. **Navigate**: Two-finger scroll to pan, scroll wheel to zoom
5. **Edit**: Double-click nodes to edit titles

## 🎮 **Controls Reference**

### **Cake Operations**
- **Click cake**: Select entire structure
- **Drag cake**: Move complete cake with all nodes
- **New Cake button**: Create new independent structure

### **Node Operations** 
- **Click node**: Select individual node
- **Double-click**: Edit node title and properties
- **Drag from node**: Create connection to another node
- **Right-click**: Context menu for node creation

### **Navigation**
- **Two-finger scroll**: Pan view (MacBook trackpad)
- **Scroll wheel**: Zoom in/out (0.1x - 5.0x)
- **Drag empty space**: Alternative pan method
- **Reset View**: Return to default position

## 🤝 **Contributing**

This project is in active development. Current focus areas:
- Graphics optimization and stability
- Data structure improvements
- Performance enhancements
- User experience refinements

## 📝 **License**

This project is licensed under the MIT License - see the LICENSE file for details.

## 🌟 **Vision**

Cosmos aims to revolutionize how we visualize and interact with complex, multi-dimensional data relationships. By moving beyond traditional flat graphs to intuitive 3D cake structures, we're building a new paradigm for knowledge management and conceptual modeling.

---

*Status: Active Development - Core features working, graphics and data structure optimizations in progress*