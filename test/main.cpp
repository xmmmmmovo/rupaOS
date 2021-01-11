#include <iostream>
#include <vector>

using namespace std;

int main() {
    vector<pair<int, int>> vec = {{1, 2}};

    for (auto &[a, b] : vec) {
        cout << a << " " << b << endl;
    }

    return 0;
}
