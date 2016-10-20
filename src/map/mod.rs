use std::slice::Iter;
use std::sync::Arc;

use ncollide::shape::*;
use nalgebra::*;

pub struct Line {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64
}

pub struct Map {
    pub start_x: f64,
    pub start_y: f64,
    pub points: Vec<Line>
}

/*
    var out = "";
    $('line').each(function () {
        out += "Line { " + "x1: " + this.x1.baseVal.value + " x2: " + this.x2.baseVal.value + " y1: " + this.y1.baseVal.value + " y2: " + this.y2.baseVal.value +  "},\n";
    }); console.log(out);
*/
impl Map {
    pub fn demo2() -> Map {
        Map {
            start_x: 1196.0,
            start_y: 908.0,
            points: vec![
                Line { x1: 1099.851806640625, x2: 1104.5926513671875, y1: 957.0370483398438, y2: 497.1851806640625},
                Line { x1: 1370.0740966796875, x2: 1367.7037353515625, y1: 964.1481323242188, y2: 440.2962951660156},
                Line { x1: 1102.22216796875, x2: 1024.0, y1: 499.5555419921875, y2: 283.8518371582031},
                Line { x1: 1365.3333740234375, x2: 1261.0369873046875, y1: 442.6666564941406, y2: 200.88888549804688},
                Line { x1: 1024.0, x2: 760.888916015625, y1: 286.22222900390625, y2: 269.629638671875},
                Line { x1: 760.888916015625, x2: 599.7036743164062, y1: 269.629638671875, y2: 452.1481628417969},
                Line { x1: 599.7036743164062, x2: 725.3333129882812, y1: 452.1481628417969, y2: 603.8518676757812},
                Line { x1: 725.3333129882812, x2: 687.4074096679688, y1: 603.8518676757812, y2: 812.4444580078125},
                Line { x1: 687.4074096679688, x2: 836.74072265625, y1: 812.4444580078125, y2: 997.3333129882812},
                Line { x1: 1261.0369873046875, x2: 1088.0, y1: 203.25926208496094, y2: 72.88888549804688},
                Line { x1: 1088.0, x2: 587.8518676757812, y1: 72.88888549804688, y2: 80.0},
                Line { x1: 585.4815063476562, x2: 229.92591857910156, y1: 80.0, y2: 395.2592468261719},
                Line { x1: 229.92591857910156, x2: 410.0740661621094, y1: 395.2592468261719, y2: 656.0},
                Line { x1: 410.0740661621094, x2: 369.77777099609375, y1: 656.0, y2: 817.1851806640625},
                Line { x1: 369.77777099609375, x2: 542.8148193359375, y1: 817.1851806640625, y2: 1073.1851806640625},
            ]
        }
    }
    pub fn demo() -> Map {
        Map {
            start_x: 3430.679012345678,//1196.0,
            start_y: 3961.283950617284,//908.0,
            points: vec![
                Line { x1: 3422.666748046875, x2: 3421.333251953125, y1: 3969.333251953125, y2: 3289.333251953125},
                Line { x1: 3730.666748046875, x2: 3714.666748046875, y1: 3966.666748046875, y2: 3282.666748046875},
                Line { x1: 3420.0, x2: 3410.666748046875, y1: 3290.666748046875, y2: 2441.333251953125},
                Line { x1: 3713.333251953125, x2: 3700.0, y1: 3284.0, y2: 2441.333251953125},
                Line { x1: 3408.0, x2: 3410.666748046875, y1: 2445.333251953125, y2: 1660.0},
                Line { x1: 3698.666748046875, x2: 3692.0, y1: 2446.666748046875, y2: 1657.3333740234375},
                Line { x1: 3410.666748046875, x2: 3284.0, y1: 1661.3333740234375, y2: 1270.6666259765625},
                Line { x1: 3693.333251953125, x2: 3542.666748046875, y1: 1658.6666259765625, y2: 1222.6666259765625},
                Line { x1: 3281.333251953125, x2: 3277.333251953125, y1: 1270.6666259765625, y2: 1077.3333740234375},
                Line { x1: 3540.0, x2: 3534.666748046875, y1: 1222.6666259765625, y2: 1034.6666259765625},
                Line { x1: 3273.333251953125, x2: 3380.0, y1: 1074.6666259765625, y2: 678.6666870117188},
                Line { x1: 3534.666748046875, x2: 3666.666748046875, y1: 1033.3333740234375, y2: 628.0},
                Line { x1: 3378.666748046875, x2: 3368.0, y1: 680.0, y2: 536.0},
                Line { x1: 3665.333251953125, x2: 3609.333251953125, y1: 628.0, y2: 434.6666564941406},
                Line { x1: 3364.0, x2: 3233.333251953125, y1: 537.3333129882812, y2: 384.0},
                Line { x1: 3608.0, x2: 3437.333251953125, y1: 433.3333435058594, y2: 244.0},
                Line { x1: 3234.666748046875, x2: 3090.666748046875, y1: 386.6666564941406, y2: 409.3333435058594},
                Line { x1: 3438.666748046875, x2: 3294.666748046875, y1: 245.3333282470703, y2: 160.0},
                Line { x1: 3092.0, x2: 2713.333251953125, y1: 404.0, y2: 414.6666564941406},
                Line { x1: 3293.333251953125, x2: 2662.666748046875, y1: 160.0, y2: 192.0},
                Line { x1: 2715.25927734375, x2: 2584.888916015625, y1: 408.8888854980469, y2: 577.1851806640625},
                Line { x1: 2660.74072265625, x2: 2461.629638671875, y1: 195.55555725097656, y2: 306.96295166015625},
                Line { x1: 2461.629638671875, x2: 2267.25927734375, y1: 306.96295166015625, y2: 638.8148193359375},
                Line { x1: 2264.888916015625, x2: 2272.0, y1: 634.0740966796875, y2: 1077.3333740234375},
                Line { x1: 2587.25927734375, x2: 2580.148193359375, y1: 574.8148193359375, y2: 1079.7037353515625},
                Line { x1: 2269.629638671875, x2: 2506.666748046875, y1: 1070.22216796875, y2: 1594.0740966796875},
                Line { x1: 2584.888916015625, x2: 2885.926025390625, y1: 1077.3333740234375, y2: 1544.2962646484375},
                Line { x1: 2506.666748046875, x2: 2198.5185546875, y1: 1591.7037353515625, y2: 2034.9630126953125},
                Line { x1: 2883.5556640625, x2: 2786.370361328125, y1: 1532.4444580078125, y2: 1750.5185546875},
                Line { x1: 2193.77783203125, x2: 2203.25927734375, y1: 2030.22216796875, y2: 2324.148193359375},
                Line { x1: 2788.74072265625, x2: 2902.5185546875, y1: 1750.5185546875, y2: 1933.0369873046875},
                Line { x1: 2198.5185546875, x2: 2212.74072265625, y1: 2309.926025390625, y2: 2793.4814453125},
                Line { x1: 2900.148193359375, x2: 2589.629638671875, y1: 1918.8148193359375, y2: 2096.592529296875},
                Line { x1: 2212.74072265625, x2: 1852.4444580078125, y1: 2788.74072265625, y2: 2807.70361328125},
                Line { x1: 2589.629638671875, x2: 2594.370361328125, y1: 2094.22216796875, y2: 3058.962890625},
                Line { x1: 1857.1851806640625, x2: 1482.6666259765625, y1: 2805.333251953125, y2: 2686.814697265625},
                Line { x1: 2592.0, x2: 2250.666748046875, y1: 3049.4814453125, y2: 3127.70361328125},
                Line { x1: 1480.2962646484375, x2: 1020.4444580078125, y1: 2682.073974609375, y2: 2129.77783203125},
                Line { x1: 2250.666748046875, x2: 1790.8148193359375, y1: 3122.962890625, y2: 3156.148193359375},
                Line { x1: 1025.1851806640625, x2: 958.8148193359375, y1: 2129.77783203125, y2: 1757.629638671875},
                Line { x1: 1793.1851806640625, x2: 1314.370361328125, y1: 3163.25927734375, y2: 3028.148193359375},
                Line { x1: 954.0740966796875, x2: 555.8518676757812, y1: 1757.629638671875, y2: 1541.9259033203125},
                Line { x1: 1319.111083984375, x2: 847.4074096679688, y1: 3035.25927734375, y2: 2537.4814453125},
                Line { x1: 555.8518676757812, x2: 55.703704833984375, y1: 1539.5555419921875, y2: 1916.4444580078125},
                Line { x1: 847.4074096679688, x2: 608.0, y1: 2542.22216796875, y2: 2049.185302734375},
                Line { x1: 48.592594146728516, x2: 36.74074172973633, y1: 1911.7037353515625, y2: 2874.073974609375},
                Line { x1: 603.25927734375, x2: 406.5185241699219, y1: 2044.4444580078125, y2: 2317.037109375},
                Line { x1: 32.790122985839844, x2: 500.543212890625, y1: 2870.91357421875, y2: 3812.74072265625},
                Line { x1: 408.8888854980469, x2: 421.5308532714844, y1: 2314.666748046875, y2: 2807.70361328125},
                Line { x1: 494.22222900390625, x2: 936.6913452148438, y1: 3800.098876953125, y2: 3980.246826171875},
                Line { x1: 427.8518371582031, x2: 775.5061645507812, y1: 2807.70361328125, y2: 3496.69140625},
                Line { x1: 939.8518676757812, x2: 2592.7900390625, y1: 3980.246826171875, y2: 3926.5185546875},
                Line { x1: 772.345703125, x2: 2554.8642578125, y1: 3484.04931640625, y2: 3594.666748046875},
                Line { x1: 2583.30859375, x2: 3117.43212890625, y1: 3923.35791015625, y2: 3768.493896484375},
                Line { x1: 2539.061767578125, x2: 2858.271484375, y1: 3588.345703125, y2: 3041.580322265625},
                Line { x1: 3111.111083984375, x2: 3212.246826171875, y1: 3762.1728515625, y2: 3139.5556640625},
                Line { x1: 2855.111083984375, x2: 3209.08642578125, y1: 3044.74072265625, y2: 3149.037109375},
                Line { x1: 3417.678955078125, x2: 3736.888916015625, y1: 3961.283935546875, y2: 3961.283935546875},
                Line { x1: 1131.875, x2: 1131.875, y1: 1251.583984375, y2: 1231.583984375},
            ]
        }
    }

    pub fn build_collision_shape(&self) -> Polyline<Pnt2<f64>> {
        let mut points: Vec<Pnt2<f64>> = Vec::new();
        let mut indices: Vec<Pnt2<usize>> = Vec::new();

        let mut i = 0;
        for line in &self.points {
            // build points
            points.push(Pnt2::new(line.x1, line.y1));
            points.push(Pnt2::new(line.x2, line.y2));

            // build indices
            indices.push(Pnt2::new(i, i+1));

            i += 2;
        }

        Polyline::new(Arc::new(points), Arc::new(indices), None, None)
    }

    pub fn iter(&self) -> Iter<Line> {
        self.points.iter()
    }
}