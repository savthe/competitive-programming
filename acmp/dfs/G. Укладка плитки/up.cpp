#include <iostream>
#include <vector>

using namespace std;

vector<vector<char>> adj;
const int N = 8;

void dfs(int y, int x, char expect) {
  if (y < 0 || y >= N || x < 0 || x >= N) return;
	if (adj[y][x] != expect) return;

  adj[y][x] = 0;
	if (expect == 'W') expect = 'B';
	else expect = 'W';
	
  dfs(y + 1, x, expect);
  dfs(y - 1, x, expect);
  dfs(y, x + 1, expect);
  dfs(y, x - 1, expect);
}

int main() {
  adj.resize(N, vector<char>(N));
  for(int i = 0; i < N; ++i) 
    for (int j = 0; j < N; ++j) 
      cin >> adj[i][j];

  int c = 0;
  for(int i = 0; i < N; ++i) {
    for (int j = 0; j < N; ++j) {
      if (adj[i][j] != 0) {
        c++;
        dfs(i, j, adj[i][j]);
      }
    }
  }
  cout << c << endl;
}
