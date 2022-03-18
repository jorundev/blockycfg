#pragma once

class Point2f
{
	private:
		float _x, _y;

	public:

		Point2f()
			: _x(0), _y(0)
		{	}

		Point2f(float x, float y)
			: _x(x), _y(y)
		{	}

		float x() const
		{
			return this->_x;
		}

		float y() const
		{
			return this->_y;
		}

		void setX(float x)
		{
			this->_x = x;
		}

		void setY(float y)
		{
			this->_y = y;
		}

		float& rx()
		{
			return this->_x;
		}

		float& ry()
		{
			return this->_y;
		}

		Point2f& operator-=(const Point2f& point)
		{
			this->_x -= point.x();
			this->_y -= point.y();
			return (*this);
		}

		Point2f& operator+=(const Point2f& point)
		{
			this->_x += point.x();
			this->_y += point.y();
			return (*this);
		}

		Point2f& operator*=(const float& factor)
		{
			this->_x += factor;
			this->_y += factor;
			return (*this);
		}
};
