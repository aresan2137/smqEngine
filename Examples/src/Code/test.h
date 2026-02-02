#pragma once

#include <Smq.h>

class Test : public smq::Component {
public:
	Test();

	void Start() override;

	void Update(float Delta) override;

private:

};


