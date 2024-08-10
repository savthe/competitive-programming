#include <iostream>
#include <vector>

using namespace std;

// Список смежности. adj[i] = вектор с друзьями i-го человека.
vector<vector<int>> adj;

// Посещенные или не посещенные вершины.
vector<bool> used;

// Решение: необходимо найти размер компоненты связности, содержащей человека S.

void dfs(int v) {
    // Посетили v.
    used[v] = true;

    // Перебираем всех друзей v и посещаем тех, которых еще не посетили.
    for (auto nb: adj[v]) 
        if (!used[nb]) dfs(nb);
}

int  main() {
    int n, s; cin >> n >> s;

    adj.resize(n+1);
    used.resize(n+1);
    for (int i = 1; i <= n; ++i) {
      for (int j = 1; j <= n; ++j) {
        int x; cin >> x;
        if (x == 1) adj[i].push_back(j);
      }
    }

	dfs(s);

	s = 0;
  // Ответ: количество посещенных вершин без одной.
	for (auto x : used) s += x;

	cout << s - 1 << endl;
}
