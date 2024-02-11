// Problem: https://codeforces.com/contest/1921/problem/G

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

using Matrix = vector<vector<int>>;

// Finds max sum in all k-triangles of the form
// xxxx
// xxx
// xx
// x
int max_sum(const Matrix &matrix, int k) {
  const int rows = matrix.size();
  const int cols = matrix[0].size();

  // Consider example matrix A and k = 2:
  // 0 1 0 1 1
  // 1 1 0 1 0
  // 0 0 1 1 0
  // 1 0 1 0 1
  // Max sum is 4, for the following triangles:
  // 0 1 x x x     0 1 0 1 1     0 1 0 x x
  // 1 1 x x 0     1 1 x x x     1 1 0 x x
  // 0 0 x 1 0     0 0 x x 0     0 0 1 x 0
  // 1 0 1 0 1     1 0 x 0 1     1 0 1 0 1
  // Note the last 'partial' triangle. There are cases when these partial
  // triangles gives max sum.

  // Matrix for horizontal prefixes. For each row in `matrix`, a row in
  // horizontal sums matrix holds prefix sums.
  Matrix horizontal_sums(rows, vector<int>(cols));
  // For matrix A:
  // 0 1 1 2 3
  // 1 2 2 3 3
  // 0 0 1 2 2
  // 1 1 2 2 3

  // Matrix for prefix sums in column (going from up to down).
  Matrix vertical_sums(rows, vector<int>(cols));
  // For matrix A:
  // 0 1 0 1 1
  // 1 2 0 2 1
  // 1 2 1 3 1
  // 2 2 2 3 2

  // Matrix for prefix sums in each diagonal (going from upper right to bottom
  // left).
  Matrix diagonal_sums(rows, vector<int>(cols));
  // For matrix A:
  // 0 1 0 1 1
  // 2 1 1 2 0
  // 1 1 3 1 0
  // 2 3 2 0 1

  // Compute prefix sum matrices.
  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      horizontal_sums[i][j] = matrix[i][j];
      vertical_sums[i][j] = matrix[i][j];
      diagonal_sums[i][j] = matrix[i][j];

      // If 'previous' prefix (a sum on prefix before this element) exists.
      if (j > 0) horizontal_sums[i][j] += horizontal_sums[i][j - 1];
      if (i > 0) vertical_sums[i][j] += vertical_sums[i - 1][j];
      if (i > 0 && j + 1 < cols)
        diagonal_sums[i][j] += diagonal_sums[i - 1][j + 1];
    }
  }

  // For each cell we compute a sum of tiangle for which current cell is upper
  // left.
  Matrix dp(rows, vector<int>(cols));
  // For matrix A with k = 2:
  // 3 3 4 4 1
  // 3 3 4 2 1
  // 2 3 3 2 1
  // 2 1 2 1 1

  // Compute sum of triangle with coordinates (0, 0) - its upper left cell.
  // Note that k might be greater than number of rows or columns.
  for (int i = 0; i <= min(k, rows - 1); ++i) {
    for (int j = 0; j <= min(k, cols - 1); ++j) {
      // If distance (hops through adjacent cells) is not greater than k.
      if (i + j <= k) dp[0][0] += matrix[i][j];
    }
  }

  // Computes sum in horizontal segment [row][col] - [row][col + k]. Handles
  // smaller segment if col + k is greater than cols - 1.
  auto compute_hor_sum = [&](int row, int col) {
    int s = horizontal_sums[row][min(cols - 1, col + k)];
    // Subtracting left col - 1 prefix if it exists.
    if (col > 0) s -= horizontal_sums[row][col - 1];
    return s;
  };

  // Computes sum in vertical segment [row][col] - [row + k][col].
  auto compute_vert_sum = [&](int row, int col) {
    int s = vertical_sums[min(rows - 1, row + k)][col];
    if (row > 0) s -= vertical_sums[row - 1][col];
    return s;
  };

  // Computes sum in diagonal segment from [row + sz][col] to [row][col + sz]
  // (going upper right). Handles smaller segment if one of endpoints is out of
  // matrix.
  auto compute_diag_sum = [&](int row, int col, int sz) {
    // d is a number of cells which don't fit into matrix vertical size.
    int d = max(0, row + sz - (rows - 1));

    // If left endpoint is out of matrix size.
    if (col + d >= cols) return 0;

    int s = diagonal_sums[row + sz - d][col + d];
    if (row > 0 && col + sz + 1 < cols)
      s -= diagonal_sums[row - 1][col + sz + 1];
    return s;
  };

  // Compute sum for 0-th row.
  for (int col = 1; col < cols; ++col) {
    // Sum in current triangle equals to the sum of triangle 1 column left minus
    // its left column, plus diagonal.
    dp[0][col] = dp[0][col - 1] - compute_vert_sum(0, col - 1) +
                 compute_diag_sum(0, col, k);
  }

  // Compute sum for other rows. Let dp[i][j] is a sum of the following
  // triangle:
  // xxxxx
  // xxxx
  // xxx
  // xx
  // x
  // Consider a triangle with upper left cell in [i + 1][j] (we denote it with
  // 'y'):
  // xxxxx
  // yyyyy
  // yyyy
  // yyy
  // yy
  // y
  // We see that it contains the x-triangle without its first line with
  // additional diagonal.
  for (int row = 1; row < rows; ++row) {
    for (int col = 0; col < cols; ++col) {
      dp[row][col] = dp[row - 1][col] - compute_hor_sum(row - 1, col) +
                     compute_diag_sum(row, col, k);
    }
  }

  // Find max sum.
  int max_sum = 0;
  for (const auto &v : dp) {
    max_sum = max(max_sum, *max_element(v.begin(), v.end()));
  }

  return max_sum;
}

// Rotates the matrix counter-clockwise:
// a b c        c z
// x y z   =>   b y
//              a x
Matrix rotate(const Matrix &matrix) {
  const auto rows = matrix.size();
  const auto cols = matrix[0].size();

  Matrix result(cols, vector<int>(rows));
  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      result[cols - j - 1][i] = matrix[i][j];
    }
  }
  return result;
}

void solve() {
  int n, m, k;
  cin >> n >> m >> k;

  Matrix matrix(n, vector<int>(m));
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < m; ++j) {
      char c;
      cin >> c;
      matrix[i][j] = c == '#';
    }
  }

  int sum = max_sum(matrix, k);

  for (int i = 0; i < 3; ++i) {
    matrix = rotate(matrix);
    sum = max(sum, max_sum(matrix, k));
  }

  cout << sum << endl;
}

int main() {
  int t;
  cin >> t;
  while (t--) {
    solve();
  }
}
