#include <iostream>
#include <vector>

using namespace std;

vector<vector<char>> adj;
int n, m;

// Считаем клетку вершиной графа, соседние клетки -- соседними вершинами. Требуется посчитать
// количество компонент связности.
void dfs(int y, int x) {
  // Если шагнули в несуществующую клетку, то выходим.
  if (y < 0 || y >= n || x < 0 || x >= m) return;

  // Заходим в клетку, если она содержит #.
  if (adj[y][x] != '#') return;

  // Затираем клетку нулем, чтобы больше в нее не заходить.
  adj[y][x] = 0;

  // Идем в соседние клетки.
  dfs(y + 1, x);
  dfs(y - 1, x);
  dfs(y, x + 1);
  dfs(y, x - 1);
}

int main() {
  cin >> n >> m;
  adj.resize(n, vector<char>(m));
  for(int i = 0; i < n; ++i) 
    for (int j = 0; j < m; ++j) 
      cin >> adj[i][j];
  
  int c = 0;
  for(int i = 0; i < n; ++i) {
    for (int j = 0; j < m; ++j) {
      // Если клетку еще не посетили, то она в новой компоненте связности.
      if (adj[i][j] == '#') {
        c++;
        dfs(i, j);
      }
    }
  }
  cout << c << endl;
}
