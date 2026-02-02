#pragma once

#include <Smq.h>

class t : public smq::Component {
public:
	t();

	void Start() override;

	void Update(float Delta) override;

private:

};


