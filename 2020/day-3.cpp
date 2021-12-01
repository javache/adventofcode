#include <iostream>
#include <string>
#include <cstdlib>
#include <vector>

int main() {
  std::vector<std::vector<bool>> map;

  std::string line;
  while(std::getline(std::cin, line)) {
    auto& row = map.emplace_back();
    row.reserve(line.size());
    for (char c : line) {
      row.push_back(c == '#');
    }
  }

  int slopes[][2] = {{1, 1}, {1, 3}, {1, 5}, {1, 7}, {2, 1}};

  int product = 1;
  for (auto& slope : slopes) {
    int x = 0;
    int trees_encountered = 0;
    for (int i = 0; i < map.size(); i += slope[0]) {
      if (map[i][x]) {
        trees_encountered++;
      }
      x = (x + slope[1]) % map[i].size();
    }
    std::cout << "Slope has " << trees_encountered << " trees" << std::endl;
    product *= trees_encountered;
  }

  std::cout << "Encountered trees: " << product << std::endl;
  return 0;
}