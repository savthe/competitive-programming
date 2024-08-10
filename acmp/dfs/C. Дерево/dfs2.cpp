#include <iostream>
#include <vector>

using namespace std;

vector<vector<int>> adj;
vector<bool> used;

void dfs(int v) {
  used[v] = true;
  for (auto nb: adj[v]) 
  if (!used[nb]) dfs(nb);
}

int  main() 
{
  int n; cin >> n;

  adj.resize(n+1);
  used.resize(n+1);
  // При вводе считаем количество ребер.
  int edges = 0;
  for (int i = 1; i <= n; ++i) {
    for (int j = 1; j <= n; ++j) {
      int x; cin >> x;
      if (x == 1) {
        adj[i].push_back(j);
        edges++;
      }
    }
  }

  // Используем DFS чтобы проверить, связный ли граф (из любой вершины можно добраться во все 
  // отсальные).
  dfs(1);
  int c = 0;
	for (auto x : used) c += x;

  // В дереве на N вершинах всегда N - 1 ребро. Так как матрица смежности симметричная, каждое
  // ребро посчитано 2 раза.
  // Проверяем, что ребер N - 1 и все вершины посещены.
	if (edges / 2 == n - 1 && c == n) cout << "YES" << endl;
	else cout << "NO" << endl;
}
