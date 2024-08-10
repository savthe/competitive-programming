#include <iostream>
#include <vector>
#include <set>
#include <algorithm>
using namespace std;

// требуется проверить ориентированный несвязный граф на наличие циклов

vector<vector<int>> adj;
vector<int> used;

// типы вершин для массива used:
// not_visited -- еще не посещали
// in_path -- мы уже прошли через эту вершину
// visited -- вершина обработана

enum {not_visited, in_path, visited};

bool dfs(int v) {
    // нашли новую вершину, отмечаем, что она содержится в пути
    used[v] = in_path;
    for (auto nb : adj[v]) {
        // если сосед nb уже содержится в пути, но нашли цикл
        if (used[nb] == in_path) return false;

        // если сосед nb не посещен, запускаем dfs(nb), если он вернул 
        // false, то где-то глубже найден цикл, возвращаем false
        if (used[nb] == not_visited && !dfs(nb)) return false;
    }
    // вершина обработана
    used[v] = visited;
    return true;
}

int main() { int n, m; cin >> n >> m;

    adj.resize(n + 1);
    used.resize(n + 1, not_visited);

    while (m--) {
        int a, b;
        cin >> a >> b;
        adj[a].push_back(b);
    }

    for (int i = 1; i <= n; ++i) {
        // запускаем dfs из каждой необработанной вершины
        if (used[i] == not_visited && !dfs(i)) {
            cout << "No" << endl;
            return 0;
        }
    }
    cout << "Yes" << endl;
}

