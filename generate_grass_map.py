import json

# Map configuration
width = 100
height = 100

# Generate tiles - all grass
tiles = []
for y in range(height):
    for x in range(width):
        tiles.append({
            "x": x,
            "y": y,
            "type": "grass"
        })

# Create map data
map_data = {
    "width": width,
    "height": height,
    "tiles": tiles
}

# Write to file
with open("assets/maps/grass_map.json", "w") as f:
    json.dump(map_data, f, indent=2)

print(f"Generated {width}x{height} grass map with {len(tiles)} tiles")