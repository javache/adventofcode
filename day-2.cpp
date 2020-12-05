#include <iostream>
#include <string>
#include <cstdlib>
#include <unordered_set>

int main() {
  int count_a = 0;
  int count_b = 0;

  std::string line;
  while(std::getline(std::cin, line)) {
    // 1-8 n: dpwpmhknmnlglhjtrbpx
    int lower_bound = std::stoi(line.substr(0, line.find("-")));
    int upper_bound = std::stoi(line.substr(line.find("-") + 1, line.find(" ")));
    char target = line.substr(line.find(":") - 1, line.find(line.find(":")))[0];
    std::string password = line.substr(line.find(":") + 2);

    int char_count = 0;
    for (char c : password) {
      if (c == target) {
        char_count++;
      }
    }

    if (char_count >= lower_bound && char_count <= upper_bound) {
      count_a++;
    }

    if ((password[lower_bound - 1] == target) ^ (password[upper_bound - 1] == target)) {
      count_b++;
    }
  }

  std::cout << count_a << " passwords are valid (1)" << std::endl;
  std::cout << count_b << " passwords are valid (2)" << std::endl;
  return 0;
}