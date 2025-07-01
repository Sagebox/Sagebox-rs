	#[allow(non_upper_case_globals)]

	#[derive(Copy, Clone)]
	pub struct Point<T>
	{
	 pub x : T,
     pub y : T,
	}
	impl Point<f32>	
	{
		pub fn f32(&self) -> (f32,f32)
		{
			(self.x,self.y)
		}
		pub fn new(_x : f32, _y : f32) -> Point<f32>
		{
			Point{ x : _x, y : _y }
		}

		pub fn fromf32(input : (f32,f32)) -> Point<f32>
		{
			Point{ x : input.0, y : input.1 }
		}

		pub fn fromi32(input : (i32,i32)) -> Point<f32>
		{
			Point{ x : input.0 as f32, y : input.1 as f32 }
		}
		pub fn default() -> Point<f32>
		{
			Point{ x : 0.0, y : 0.0 }
		}

		pub fn within_rect(&self,top_left : Point<f32>, bottom_right : Point<f32>) -> bool
		{
			self.x >= top_left.x && self.y >= top_left.y && self.x < bottom_right.x && self.y < bottom_right.y
		}
	}
	

impl Point<f64>	
	{
		pub fn f64(&self) -> (f64,f64)
		{
			(self.x,self.y)
		}
		pub fn f32(&self) -> (f32,f32)
		{
			(self.x as f32,self.y as f32)
		}
		pub fn new(_x : f64, _y : f64) -> Point<f64>
		{
			Point{ x : _x, y : _y }
		}

		pub fn fromf32(input : (f32,f32)) -> Point<f64>
		{
			Point{ x : input.0 as f64, y : input.1 as f64 }
		}

		pub fn fromf64(input : (f64,f64)) -> Point<f64>
		{
			Point{ x : input.0, y : input.1 }
		}

		pub fn fromi32(input : (i32,i32)) -> Point<f64>
		{
			Point{ x : input.0 as f64, y : input.1 as f64 }
		}
		pub fn default() -> Point<f64>
		{
			Point{ x : 0.0, y : 0.0 }
		}
		pub fn within_rect(&self,top_left : Point<f64>, bottom_right : Point<f64>) -> bool
		{
			self.x >= top_left.x && self.y >= top_left.y && self.x < bottom_right.x && self.y < bottom_right.y
		}
	}

	impl Point<i32>	{
		pub fn i32(&self) -> (i32,i32)
		{
			(self.x,self.y)
		}
		pub fn f32(&self) -> (f32,f32)
		{
			(self.x as f32,self.y as f32)
		}
		pub fn f64(&self) -> (f64,f64)
		{
			(self.x as f64,self.y as f64)
		}
		pub fn to_pi32(&self) -> Point<i32>
		{
			Point::<i32>::new(self.x,self.y)
		}
		pub fn to_pf32(&self) -> Point<f32>
		{
			Point::<f32>::new(self.x as f32,self.y as f32)
		}
		pub fn to_pf64(&self) -> Point<f64>
		{
			Point::<f64>::new(self.x as f64,self.y as f64)
		}
		pub fn new(_x : i32, _y : i32) -> Point<i32>
		{
			Point{ x : _x, y : _y }
		}
		pub fn default() -> Point<i32>
		{
			Point{ x : 0, y : 0 }
		}
		pub fn fromi32(input : (i32,i32)) -> Point<i32>
		{
			Point{ x : input.0, y : input.1 }
		}
		pub fn within_rect(&self,top_left : Point<i32>, bottom_right : Point<i32>) -> bool
		{
			self.x >= top_left.x && self.y >= top_left.y && self.x < bottom_right.x && self.y < bottom_right.y
		}
	}

	// ***************************** Point<f64> *****************************
	
	impl std::ops::Add<Point<f64>> for Point<f64> { type Output = Point<f64>; fn add(self, _rhs: Point<f64>) -> Point<f64> { Point{ x: self.x + _rhs.x, y: self.y + _rhs.y} } }	
	impl std::ops::Sub<Point<f64>> for Point<f64> { type Output = Point<f64>; fn sub(self, _rhs: Point<f64>) -> Point<f64> { Point{ x: self.x - _rhs.x, y: self.y - _rhs.y} } }	
	impl std::ops::Mul<Point<f64>> for Point<f64> { type Output = Point<f64>; fn mul(self, _rhs: Point<f64>) -> Point<f64> { Point{ x: self.x * _rhs.x, y: self.y * _rhs.y} } }
	impl std::ops::Div<Point<f64>> for Point<f64> { type Output = Point<f64>; fn div(self, _rhs: Point<f64>) -> Point<f64> { Point{ x: self.x / _rhs.x, y: self.y / _rhs.y} } }

	impl std::ops::Add<Point<i32>> for Point<f64> { type Output = Point<f64>; fn add(self, _rhs: Point<i32>) -> Point<f64> { Point{ x: self.x + _rhs.x as f64, y: self.y + _rhs.y as f64} } }	
	impl std::ops::Sub<Point<i32>> for Point<f64> { type Output = Point<f64>; fn sub(self, _rhs: Point<i32>) -> Point<f64> { Point{ x: self.x - _rhs.x as f64, y: self.y - _rhs.y as f64} } }	
	impl std::ops::Mul<Point<i32>> for Point<f64> { type Output = Point<f64>; fn mul(self, _rhs: Point<i32>) -> Point<f64> { Point{ x: self.x * _rhs.x as f64, y: self.y * _rhs.y as f64} } }
	impl std::ops::Div<Point<i32>> for Point<f64> { type Output = Point<f64>; fn div(self, _rhs: Point<i32>) -> Point<f64> { Point{ x: self.x / _rhs.x as f64, y: self.y / _rhs.y as f64} } }

	impl std::ops::Add<Point<f64>> for (f64,f64) { type Output = (f64,f64); fn add(self, _rhs: Point<f64>) -> (f64,f64) { (self.0 + _rhs.x, self.1 + _rhs.y) } }	
	impl std::ops::Sub<Point<f64>> for (f64,f64) { type Output = (f64,f64); fn sub(self, _rhs: Point<f64>) -> (f64,f64) { (self.0 - _rhs.x, self.1 - _rhs.y) } }	

	impl std::ops::Add<f64> for Point<f64> { type Output = Point<f64>; fn add(self, _rhs: f64) -> Point<f64> { Point{ x: self.x + _rhs, y: self.y + _rhs} } }	
	impl std::ops::Sub<f64> for Point<f64> { type Output = Point<f64>; fn sub(self, _rhs: f64) -> Point<f64> { Point{ x: self.x - _rhs, y: self.y - _rhs} } }	
	impl std::ops::Div<f64> for Point<f64> { type Output = Point<f64>; fn div(self, _rhs: f64) -> Point<f64> { Point{ x: self.x / _rhs, y: self.y / _rhs} } }	
	impl std::ops::Mul<f64> for Point<f64> { type Output = Point<f64>; fn mul(self, _mul: f64) -> Point<f64> { Point{ x: self.x * _mul, y: self.y * _mul} } }	

	impl std::ops::Add<i32> for Point<f64> { type Output = Point<f64>; fn add(self, _rhs: i32) -> Point<f64> { Point{ x: self.x + _rhs as f64, y: self.y + _rhs as f64} } }
	impl std::ops::Mul<i32> for Point<f64> { type Output = Point<f64>; fn mul(self, _mul: i32) -> Point<f64> { Point{ x: self.x * _mul as f64, y: self.y * _mul as f64} } }	

	impl std::ops::Add<i64> for Point<f64> { type Output = Point<f64>; fn add(self, _rhs: i64) -> Point<f64> { Point{ x: self.x + _rhs as f64, y: self.y + _rhs as f64} } }
	impl std::ops::Mul<i64> for Point<f64> { type Output = Point<f64>; fn mul(self, _mul: i64) -> Point<f64> { Point{ x: self.x * _mul as f64, y: self.y * _mul as f64} } }	


	impl std::ops::AddAssign<Point<f64>> for Point<f64> { fn add_assign(&mut self, _rhs: Point<f64>) { self.x += _rhs.x; self.y += _rhs.y; } }	
	impl std::ops::AddAssign<Point<i32>> for Point<f64> { fn add_assign(&mut self, _rhs: Point<i32>) { self.x += _rhs.x as f64; self.y += _rhs.y as f64; } }	
	impl std::ops::AddAssign<Point<f32>> for Point<f64> { fn add_assign(&mut self, _rhs: Point<f32>) { self.x += _rhs.x as f64; self.y += _rhs.y as f64; } }	


	// ***************************** Point<f32> *****************************
	
	impl std::ops::Add<Point<f32>> for Point<f32> { type Output = Point<f32>; fn add(self, _rhs: Point<f32>) -> Point<f32> { Point{ x: self.x + _rhs.x, y: self.y + _rhs.y} } }	
	impl std::ops::Sub<Point<f32>> for Point<f32> { type Output = Point<f32>; fn sub(self, _rhs: Point<f32>) -> Point<f32> { Point{ x: self.x - _rhs.x, y: self.y - _rhs.y} } }	
	impl std::ops::Mul<Point<f32>> for Point<f32> { type Output = Point<f32>; fn mul(self, _rhs: Point<f32>) -> Point<f32> { Point{ x: self.x * _rhs.x, y: self.y * _rhs.y} } }
	impl std::ops::Div<Point<f32>> for Point<f32> { type Output = Point<f32>; fn div(self, _rhs: Point<f32>) -> Point<f32> { Point{ x: self.x / _rhs.x, y: self.y / _rhs.y} } }

	impl std::ops::Add<Point<i32>> for Point<f32> { type Output = Point<f32>; fn add(self, _rhs: Point<i32>) -> Point<f32> { Point{ x: self.x + _rhs.x as f32, y: self.y + _rhs.y as f32} } }	
	impl std::ops::Sub<Point<i32>> for Point<f32> { type Output = Point<f32>; fn sub(self, _rhs: Point<i32>) -> Point<f32> { Point{ x: self.x - _rhs.x as f32, y: self.y - _rhs.y as f32} } }	
	impl std::ops::Mul<Point<i32>> for Point<f32> { type Output = Point<f32>; fn mul(self, _rhs: Point<i32>) -> Point<f32> { Point{ x: self.x * _rhs.x as f32, y: self.y * _rhs.y as f32} } }
	impl std::ops::Div<Point<i32>> for Point<f32> { type Output = Point<f32>; fn div(self, _rhs: Point<i32>) -> Point<f32> { Point{ x: self.x / _rhs.x as f32, y: self.y / _rhs.y as f32} } }

	impl std::ops::Add<Point<f32>> for (f32,f32) { type Output = (f32,f32); fn add(self, _rhs: Point<f32>) -> (f32,f32) { (self.0 + _rhs.x, self.1 + _rhs.y) } }	
	impl std::ops::Sub<Point<f32>> for (f32,f32) { type Output = (f32,f32); fn sub(self, _rhs: Point<f32>) -> (f32,f32) { (self.0 - _rhs.x, self.1 - _rhs.y) } }	

	impl std::ops::Add<f32> for Point<f32> { type Output = Point<f32>; fn add(self, _rhs: f32) -> Point<f32> { Point{ x: self.x + _rhs, y: self.y + _rhs} } }	
	impl std::ops::Sub<f32> for Point<f32> { type Output = Point<f32>; fn sub(self, _rhs: f32) -> Point<f32> { Point{ x: self.x - _rhs, y: self.y - _rhs} } }	
	impl std::ops::Div<f32> for Point<f32> { type Output = Point<f32>; fn div(self, _rhs: f32) -> Point<f32> { Point{ x: self.x / _rhs, y: self.y / _rhs} } }	
	impl std::ops::Mul<f32> for Point<f32> { type Output = Point<f32>; fn mul(self, _mul: f32) -> Point<f32> { Point{ x: self.x * _mul, y: self.y * _mul} } }	

	impl std::ops::Add<i32> for Point<f32> { type Output = Point<f32>; fn add(self, _rhs: i32) -> Point<f32> { Point{ x: self.x + _rhs as f32, y: self.y + _rhs as f32} } }
	impl std::ops::Mul<i32> for Point<f32> { type Output = Point<f32>; fn mul(self, _mul: i32) -> Point<f32> { Point{ x: self.x * _mul as f32, y: self.y * _mul as f32} } }	


	impl std::ops::AddAssign<Point<f32>> for Point<f32> { fn add_assign(&mut self, _rhs: Point<f32>) { self.x += _rhs.x; self.y += _rhs.y; } }	
	impl std::ops::AddAssign<Point<i32>> for Point<f32> { fn add_assign(&mut self, _rhs: Point<i32>) { self.x += _rhs.x as f32; self.y += _rhs.y as f32; } }	

// ***************************** Point<i32> *****************************

	impl std::ops::Add<Point<i32>> for Point<i32> { type Output = Point<i32>; fn add(self, _rhs: Point<i32>) -> Point<i32> { Point{ x: self.x + _rhs.x, y: self.y + _rhs.y}  } }
	impl std::ops::Sub<Point<i32>> for Point<i32> { type Output = Point<i32>; fn sub(self, _rhs: Point<i32>) -> Point<i32> { Point{ x: self.x - _rhs.x, y: self.y - _rhs.y}  } }

	impl std::ops::Add<(i32,i32)> for Point<i32>  { type Output = Point<i32>; fn add(self, _rhs: (i32,i32))  -> Point<i32> { Point{ x: self.x + _rhs.0, y: self.y + _rhs.1 } } }
	impl std::ops::Sub<(i32,i32)> for Point<i32>  { type Output = Point<i32>; fn sub(self, _rhs: (i32,i32))  -> Point<i32> { Point{ x: self.x - _rhs.0, y: self.y - _rhs.1 } } }

	impl std::ops::Add<i32> for Point<i32> 		  { type Output = Point<i32>; fn add(self, _rhs: i32) -> Point<i32> { Point{ x: self.x + _rhs, y: self.y + _rhs} } }	
	impl std::ops::Sub<i32> for Point<i32> 		  { type Output = Point<i32>; fn sub(self, _rhs: i32) -> Point<i32> { Point{ x: self.x - _rhs, y: self.y - _rhs} } }
