#version 300 es

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

uniform mat4 P;
uniform mat4 V;

out vec3 vColor;


/*
hj_utils.glsl
*/

//==========================================================
// 常量定义
//==========================================================

#define PI 3.14159265

//==========================================================
// 通用函数
//==========================================================

/*
@brief: 求 x 的 y 次幂
@return: x^y
genType pow(genType x, genType y);

@brief: 自然常数e为底的指数函数
@return: e^x
genType exp(genType x);

@brief: 对数函数
genType log(genType x);

@brief: 平方根函数
genType sqrt(genType x);
genDType sqrt(genDType x);

@brief: 求 x % y
@return: x - y * floor(x / y)
@example: y = mod(x, 0.5); // 返回 x 对 0.5 取模的值
genType mod(genType x, float y);
genType mod(genType x, genType y);

@brief: 仅仅返回数的小数部分
genType fract(genType x);
genDType fract(genDType x);

@brief: 向正无穷取整
genType ceil(genType x);
genDType ceil(genDType x);

@brief: 向负无穷取整
genType floor(genType x);
genDType floor(genDType x);

@brief: 提取 x 的正负号
@return: x < 0.0, 返回 -1.0
         x = 0.0, 返回 0.0
         x > 0.0, 返回 +1.0
genType sign(genType x);
genIType sign(genIType x);
genDType sign(genDType x);

@brief: 返回 x 的绝对值
genType abs(genType x);
genIType abs(genIType x);
genDType abs(genDType x);

@brief: 把 x 的值限制在 minVal 和 maxVal 之间
@return: min(max(x, minVal), maxVal)
@example: y = clamp(x, 0.0, 1.0); // 把 x 的值限制在 0.0 到 1.0
genType clamp(genType x, genType minVal, genType maxVal);

@brief: 返回 x 和 y 之间的较小者
genType min(genType x, genType y);

@brief: 返回 x 和 y 之间的较大者
genType max(genType x, genType y);

@brief: 混合两个颜色
@return: 返回 x * (1 - a) + y * a
genType mix(genType x, genType y, genType a);

@brief: 插值函数需要输入两个参数。第一个是极限或阈值，第二个是我们想要检测或通过的值。
@return: x < edge，返回 0.0
         x > edge，则返回 1.0
genType step(genType edge, genType x);
genType step(float edge, genType x);
genDType step(genDType edge, genDType x);
genDType step(double edge, genDType x);

@brief: 当给定一个范围的上下限和一个数值，这个函数会在已有的范围内给出插值。
前两个参数规定转换的开始和结束点，第三个是给出一个值用来插值。
等价于：
    genType t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
@param edge0: 下限
@param edge1: 上限
@return: x <= edge0，返回 edge0
         x >= edge1，返回 edge1
         edge0 < x < edge1，则执行 0~1 之间的平滑Hermite插值
@注意点：edge0 >= edge1，行为未定义
genType smoothstep(genType edge0, genType edge1, genType x);
genType smoothstep(float edge0, float edge1, genType x);
genDType smoothstep(genDType edge0, genDType edge1, genDType x);
genDType smoothstep(double edge0, double edge1, genDType x);

@brief: 计算向量的长度
float length(genType x);
double length(genDType x);

@brief: 计算两点之间的距离，length(p0 - p1)
float distance(genType p0, genType p1);
double distance(genDType p0, genDType p1);

@brief: 计算两个向量之间的点积
@return: x[0]*y[0] + x[1]*y[1] + ...
float dot(genType x, genType y);
double dot(	genDType x, genDType y);

@brief: 计算两个向量之间的叉积
vec3 cross(vec3 x, vec3 y);
dvec3 cross(dvec3 x, dvec3 y);

@brief: 返回和向量 v 同方向的单位向量（长度为1）
genType normalize(genType v);
genDType normalize(genDType v);

@return: 如果 dot(Nref, I) < 0, 返回 N
         否则，返回 -N
genType faceforward(genType N, genType I, genType Nref);
genDType faceforward(genDType N, genDType I, genDType Nref);

@brief: 计算反射向量
@param I: 这个可能是入射的向量吧
@param N: 法向量
@return: I - 2.0 * dot(N, I) * N
genType reflect(genType I, genType N);
genDType reflect(genDType I, genDType N);

@brief: 计算折射向量
@param I: 这个可能是入射的向量吧
@param N: 法向量
@param eta: 折射率
genType refract(genType I, genType N, float eta);
genDType refract(genDType I, genDType N, float eta);


向量相关的函数：lessThan(), lessThanEqual(), greaterThan(), greaterThanEqual(), equal() and notEqual()。

@brief: returns a boolean vector in which each element i is computed as x[i] < y[i].
bvec lessThan(vec x, vec y);
bvec lessThan(ivec x, ivec y);
bvec lessThan(uvec x, uvec y);

@brief: 判断两个向量是否相等
@return: equal returns a boolean vector in which each element i is computed as x[i] == y[i].
bvec equal(vec x, vec y);
bvec equal(ivec x, ivec y);
bvec equal(uvec x, uvec y);

@brief: 检查向量 bvec 中是否存在为 true 的元素
bool any(bvec x);

@brief: 检查向量 bvec 中的元素是否全部为 true 
bool all(bvec x);

@brief: 对向量中的每个元素进行取反操作
bvec not(bvec x);

vec4 vector;
vector[0] = vector.r = vector.x = vector.s;
vector[1] = vector.g = vector.y = vector.t;
vector[2] = vector.b = vector.z = vector.p;
vector[3] = vector.a = vector.w = vector.q;


@brief: 求逆矩阵
mat2 inverse(mat2 m);
mat3 inverse(mat3 m);
mat4 inverse(mat4 m);
*/


/*
坐标轴
Y
|
|
|_ _ _ _ _ _ X
(0,0)
*/


/*
角度(angle) <--> 弧度(radians)
360   =   2π
180   =   π
-1 <= sin(θ) <= 1
-1 <= cos(θ) <= 1

弧度 = 角度 / 180.0 * PI
角度= 弧度 / PI * 180.0

@param x: 弧度
sin(x)
cos(x)

@brief: 将角度转换为弧度
genType radians(genType degrees);

@brief: 将弧度转换为角度
genType degrees(genType radians);
*/


/*
数学知识点：复数
i^2 = -1
i = sqrt(-1)
(-i)^2 = -1
sqrt(-a) = sqrt(a)i
|x + yi| = |x - yi|

# 加法
(a + bi) + (c + di) = (a + c) + (b + d)i

# 减法
(a + bi) - (c + di) = (a - c) + (b - d)i

# 乘法
(x + yi)*(x - yi)
= x^2 - xyi + xyi - y^2*i^2
= x^2 + y^2

(a + bi) * (c + di)
= ac + adi + bci + bdi^2
= (ac - bd) + (bc + ad)*i

# 除法
x + yi = (a + bi) / (c + di)
       = [ (a + bi) * (c - di) ] / [ (c + di) * (c - di)]
       = [ (ac + bd) + (bc -ad)i ] / (c^2 + d^2)

z1 + z2 = z2 + z1
z1 * z2 = z2 * z1
(z1 + z2) + z3 = z1 + (z2 + z3)
(z1 * z2) * z3 = z1 * (z2 * z3)
z1 * (z2 + z3) = z1*z2 + z1*z3
*/


/*
常用颜色
vec3 black = vec3(0, 0, 0);
vec3 red = vec3(1, 0, 0);
vec3 green = vec3(0, 1, 0);
vec3 blue = vec3(0, 0, 1);
vec3 yellow = vec3(1, 1, 0);
vec3 magenta = vec3(1, 0, 1);
vec3 cyan = vec3(0, 1, 1);
vec3 white = vec3(1, 1, 1);
*/
vec3 black() { return vec3(0, 0, 0); }
vec3 red() { return vec3(1, 0, 0); }
vec3 green() { return vec3(0, 1, 0); }
vec3 blue() { return vec3(0, 0, 1); }
vec3 yellow() { return vec3(1, 1, 0); }
vec3 magenta() { return vec3(1, 0, 1); } // 品红
vec3 cyan() { return vec3(0, 1, 1); } // 青色
vec3 white() { return vec3(1, 1, 1); }

/*
@brief: 把整数16进制格式的颜色转换为归一化的 vec3(R, G, B) 向量
@param hex: 整数16进制格式的颜色，例如 0x00FF00
@return: 归一化的 vec3(R, G, B) 向量
    R, G, B 的单位都是 float，取值范围：[0.0, 1.0]
*/
vec3 parseHexColor(int hex) {
    int rInt = (hex / 256 / 256) % 256;
    int gInt = (hex / 256      ) % 256;
    int bInt = (hex            ) % 256;

    // 归一化
    float r = float(rInt) / 255.0;
    float g = float(gInt) / 255.0;
    float b = float(bInt) / 255.0;

    return vec3(r, g, b);
}

/*
@注意：可以直接使用 distance 函数
@brief: 计算两个点之间的距离
@param p1: 第一个点 (x, y)
@param p2: 第二个点 (x, y)
@return: 返回两个点之间的距离(float)
*/
float twoPointDistance(vec2 p1, vec2 p2) {
    float dx = p1[0] - p2[0];
    float dy = p1[1] - p2[1];
    return sqrt(dx * dx + dy * dy);
}

/*
@brief: 判断浮点数 a 和 b 是否相等
@return: true 相等
         false 不相等
*/
bool float_equal(float a, float b) {
    return abs(a - b) < 0.000001;
}

/*
@brief: 判断点 point 是否在圆内部
@param point: 点 (x, y)
@param center: 圆心 (x, y)
@param radius: 圆的半径
@return true: 点在圆内部
        false: 点不在圆内部
*/
bool isPointInCircle(vec2 point, vec2 center, float radius) {
    return distance(point, center) <= radius;
}

/*
@brief: 判断点 point 是否在正方形内部
@param point: 点 (x, y)
@param center: 正方形的中心位置 (x, y)
@param radius: 正方形的中心位置距离上下左右的长度
@return true: 点在正方形内部
        false: 点不在正方形内部
*/
bool isPointInRect(vec2 point, vec2 center, float radius) {
    float x1 = center[0] - radius;
    float x2 = center[0] + radius;
    
    float y1 = center[1] - radius;
    float y2 = center[1] + radius;

    return point[0] >= x1 && point[0] <= x2 &&
           point[1] >= y1 && point[1] <= y2;
}

/*
@brief: 计算三角形面积
    海伦公式： s = sqrt(p*(p-a)*(p-b)*(p-c))
    a, b, c 是三条边的长度，p = (a + b + c) / 2
@param v1, v2, v3: 三角形的三个点
@return: 返回三角形面积
*/
float calcTriangleArea(vec2 v1, vec2 v2, vec2 v3) {
    float a = twoPointDistance(v1, v2);
    float b = twoPointDistance(v2, v3);
    float c = twoPointDistance(v3, v1);
    float p = (a + b + c) / 2.0;
    return sqrt(p * (p - a) * (p - b) * (p - c));
}

/*
@brief: 判断一个点是否在三角形内部
    通过判断 s(ABP) + s(ACP) + s(BCP) 和 s(ABC) 是否相等
    来判断点 P 是否在三角形内部
@param point: 点 (x, y)
@param v1, v2, v3: 三角形的三个点
@return true: 点 point 在三角形内部
        false: 点 point 不在三角形内部
*/
bool isPointInTriangle1(vec2 point, vec2 v1, vec2 v2, vec2 v3) {
    float s1 = calcTriangleArea(v1, v2, point);
    float s2 = calcTriangleArea(v2, v3, point);
    float s3 = calcTriangleArea(v3, v1, point);
    float s = calcTriangleArea(v1, v2, v3);
    return float_equal(s1 + s2 + s3, s);
}

/*
向量a
向量b
θ 是向量a和向量b的夹角
点乘 a*b = b*a = |a|*|b|*cos(θ) ，结果是一个数字
叉乘 a*b = |a|*|b|*sin(θ)*单位向量，结果是一个向量，单位向量用右手法则确定方向
叉乘 a*b 和 b*a 方向相反


i,j,k 分别表示 x, y, z 轴上面的单位向量
a = 5i - 6j + 3k  = (5, -6, 3)
b = -2i + 7j + 4k = (-2, 7, 4)
点乘 a * b = (5 * -2) + (-6 * 7) + (3 * 4)
           = -10 - 42 + 12
           = -40 

叉乘 a * b =
| i  j  k |
| 5 -6  3 |
|-2  7  4 |
=
| -6 3 | * i - |  5 3 | * j + |  5 -6 | * k
|  7 4 |       | -2 4 |       | -2  7 |
= (-24 - 21)*i - (20 - -6)*j + (35 - 12)*k
= -35i - 26j + 23k
*/

/*
@brief: 计算向量 p3p1 叉乘 p3p2
因为是二维的，所以 z 轴上的长度就是 0
p1: (p1.x, p1.y, 0)
p2: (p2.x, p2.y, 0)
p3: (p3.x, p3.y, 0)
向量 p3p2 = (p2.x - p3.x)*i + (p2.y - p3.y)*j + 0*k
向量 p3p1 = (p1.x - p3.x)*i + (p1.y - p3.y)*j + 0*k
i,j,k 分别表示 x, y, z 轴上面的单位向量

叉乘 p3p2 * p3p1 = 
|       i              j        k |
| (p2.x - p3.x)  (p2.y - p3.y)  0 |
| (p1.x - p3.x)  (p1.y - p3.y)  0 |
= [ (p2.x - p3.x)*(p1.y - p3.y) - (p2.y - p3.y) * (p1.x - p3.x) ] * k

叉乘 p3p1 * p3p2 = 
|       i              j        k |
| (p1.x - p3.x)  (p1.y - p3.y)  0 |
| (p2.x - p3.x)  (p2.y - p3.y)  0 |
= [ (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y) ] * k

@return: 返回的结果是结果向量 k 前面的系数
*/
float sign(vec2 p1, vec2 p2, vec2 p3) {
    return (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y);
}

/*
@brief: 判断一个点是否在三角形内部
@param point: 点 (x, y)
@param v1, v2, v3: 三角形的三个点
@return true: 点 point 在三角形内部
        false: 点 point 不在三角形内部
*/
bool isPointInTriangle2(vec2 point, vec2 v1, vec2 v2, vec2 v3) {
    float d1 = sign(point, v1, v2);
    float d2 = sign(point, v2, v3);
    float d3 = sign(point, v3, v1);

    bool has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0); // 存在负数
    bool has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0); // 存在正数

    return !(has_neg && has_pos);
}

/*
@brief: 判断一个点是否在椭圆内部
@param point: 点 (x, y)
@param c1, c2: 椭圆的两个定点
@param totalDistance: 点 point 到 c1, c2 的距离之和
@return true: 点 point 在椭圆内部
        false: 点 point 不在椭圆内部
*/
bool isPointInEllipse(vec2 point, vec2 c1, vec2 c2, float totalDistance) {
    return distance(point, c1) + distance(point, c2) <= totalDistance;
}

//==========================================================
// 2D 操作
//==========================================================

/*
@brief: 平移
@param point: 二维平面上的点
@param displacement: 平移的距离(x, y)
@return: (point.x + displacement.x, point.y + displacement.y)
*/
vec2 translate2D(vec2 point, vec2 displacement) {
  return point + displacement;
}

/*
@brief: 缩放
@param point: 二维平面上的点
@param ratio: 缩放比例，取值范围 (0.0, 1.0]
@return: 返回缩放之后的点
*/
vec2 scale2D(vec2 point, vec2 ratio) {
  return point * ratio;
}

/*
@brief: 沿着z轴逆时针旋转
@param point: 二维平面上的点 (x, y)
@param deg: 旋转的角度，取值范围 [0.0, 360.0+]
@return: 返回旋转之后的点
*/
vec2 rotate2D_z(vec2 point, float deg) {
  float x0 = point[0];
  float y0 = point[1];

  float rad = deg / 180.0 * PI;

  float x1 = x0 * cos(rad) - y0 * sin(rad);
  float y1 = x0 * sin(rad) + y0 * cos(rad);
  return vec2(x1, y1);
}

//==========================================================
// 3D 操作
//==========================================================

/*
@brief: 沿着 x 轴逆时针旋转
@param point: 三维平面上的点 (x, y, z)
@param deg: 旋转的角度，取值范围 [0.0, 360.0+]
@return: 返回旋转之后的点
*/
vec3 rotate3D_x(vec3 point, float deg) {
  float x0 = point[0];
  float y0 = point[1];
  float z0 = point[2];

  float rad = deg / 180.0 * PI;

  float x1 = x0;
  float y1 = y0 * cos(rad) - z0 * sin(rad);
  float z1 = y0 * sin(rad) + z0 * cos(rad);
  return vec3(x1, y1, z1);
}

/*
@brief: 沿着 y 轴逆时针旋转
@param point: 三维平面上的点 (x, y, z)
@param deg: 旋转的角度，取值范围 [0.0, 360.0+]
@return: 返回旋转之后的点
*/
vec3 rotate3D_y(vec3 point, float deg) {
  float x0 = point[0];
  float y0 = point[1];
  float z0 = point[2];

  float rad = deg / 180.0 * PI;

  float x1 = x0 * cos(rad) + z0 * sin(rad);
  float y1 = y0;
  float z1 = -x0 * sin(rad) + z0 * cos(rad);
  return vec3(x1, y1, z1);
}

/*
@brief: 沿着 z 轴逆时针旋转
@param point: 三维平面上的点 (x, y, z)
@param deg: 旋转的角度，取值范围 [0.0, 360.0+]
@return: 返回旋转之后的点
*/
vec3 rotate3D_z(vec3 point, float deg) {
  float x0 = point[0];
  float y0 = point[1];
  float z0 = point[2];

  float rad = deg / 180.0 * PI;

  float x1 = x0 * cos(rad) - y0 * sin(rad);
  float y1 = x0 * sin(rad) + y0 * cos(rad);
  float z1 = z0;
  return vec3(x1, y1, z1);
}

/*
@brief: 构造平移矩阵
*/
mat4 Gen3DMatTranslate(vec3 v) {
  return mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    v[0], v[1], v[2], 1.0
  );
}

/*
@brief: 构造缩放矩阵
*/
mat4 Gen3DMatScale(vec3 v) {
  return mat4(
    v[0], 0.0, 0.0, 0.0,
    0.0, v[1], 0.0, 0.0,
    0.0, 0.0, v[2], 0.0,
    0.0, 0.0, 0.0, 1.0
  );
}

/*
@brief: 构造 X 轴旋转矩阵
@param rad: 弧度，取值范围 [0, 2π]
*/
mat4 Gen3DMatRotateX(float rad) {
  float c = cos(rad);
  float s = sin(rad);

  return mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, c, s, 0.0,
    0.0, -s, c, 0.0,
    0.0, 0.0, 0.0, 1.0
  );
}

/*
@brief: 构造 Y 轴旋转矩阵
@param rad: 弧度，取值范围 [0, 2π]
*/
mat4 Gen3DMatRotateY(float rad) {
  float c = cos(rad);
  float s = sin(rad);

  return mat4(
    c, 0.0, -s, 0.0,
    0.0, 1.0, 0.0, 0.0,
    s, 0.0, c, 0.0,
    0.0, 0.0, 0.0, 1.0
  );
}

/*
@brief: 构造 Z 轴旋转矩阵
@param rad: 弧度，取值范围 [0, 2π]
*/
mat4 Gen3DMatRotateZ(float rad) {
  float c = cos(rad);
  float s = sin(rad);

  return mat4(
    c, s, 0.0, 0.0,
    -s, c, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
  );
}


void main(void) {
  mat4 rx = Gen3DMatRotateX(radians(0.0));
  mat4 ry = Gen3DMatRotateY(radians(0.0));
  mat4 rz = Gen3DMatRotateZ(radians(0.0));
  mat4 r = rz * ry * rx;

  mat4 s = Gen3DMatScale(vec3(3.0, 3.0, 3.0));
  mat4 t = Gen3DMatTranslate(vec3(0.0, 0.0, 0.0));
  mat4 M = t * s * r;

  gl_Position = P * V * M * vec4(position, 1.0);
  vColor = color;
}
