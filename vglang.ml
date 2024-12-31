enum Angle { deg(float), grad(float), rad(float) }

data Rgb(ubyte,ubyte,ubyte);

enum Length { em(float), ex(float), px(float), inch(float), cm(float), mm(float), pt(float), pc(float), percent(float)}

data Iri(string);

enum FuncIri { Iri(Iri), Path(string) }

data Point(float,float);

data Percent(float);

enum Paint { Color(Rgb), Server(FuncIri) }

data NumberOptNumber(float, optional float);

enum Coordinate { UserSpaceOnUse, ObjectBoundingBox}

enum Transform {
    Translate(float,float), 
    Matrix([float;6]), 
    Scale(float,optional float), 
    Rotate { angle: float, cx: float, cy: float },
    SkewX(float),
    SkewY(float),
}

enum Channel { R,G,B,A }

enum ClipRule { Nonzero, EvenOdd }