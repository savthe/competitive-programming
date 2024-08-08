#include <iostream>
using namespace std;
int main()
{
	uint64_t n, x, y;
	cin >> n >> x >> y;
	
	// Сделаем первую копию за min(x,y), осталась n-1 копия
	// Из уравнения: t/x + t/y = n-1, t - искомое время
	uint64_t t = (n-1)*x*y/(x+y);
	
	while(t/x + t/y < n-1) ++t;

	cout << t + min(x,y) << endl;
}

