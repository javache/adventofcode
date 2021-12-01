#include <iostream>
#include <string>
#include <unordered_set>

int main() {
  const int SUM = 2020;
  std::unordered_set<int> seen;

  std::string line;
  while(std::getline(std::cin, line)) {
    int num = std::stoi(line);

    {
      int target = SUM - num;
      auto it = seen.find(target);
      if (it != seen.end()) {
        int product = num * *it;
        std::cout << num << " * " << *it << " = " << product << std::endl;
      }
    }

    {
      for (auto it = seen.begin(); it != seen.end(); ++it) {
        int target = SUM - num - *it;
        auto it2 = seen.find(target);
        if (it2 != seen.end() && it2 != it) {
          int product = num * *it * *it2;
          std::cout << num << " * " << *it << " * " << *it2 << " = " << product << std::endl;
        }
      }
    }

    seen.insert(num);
  }
  return 0;
}