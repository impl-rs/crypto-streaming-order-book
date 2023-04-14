use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::order_book::{LevelBuilder, OrderBookBuilder};

pub fn get_binance_order_book_builder() -> OrderBookBuilder<Binance> {
    OrderBookBuilder {
        bids: vec![
            LevelBuilder::new(0.068426, 23.8545),
            LevelBuilder::new(0.068425, 4.7825),
            LevelBuilder::new(0.068424, 1.0957),
            LevelBuilder::new(0.068423, 0.4643),
            LevelBuilder::new(0.068422, 0.0662),
            LevelBuilder::new(0.068421, 0.0034),
            LevelBuilder::new(0.06842, 1.3),
            LevelBuilder::new(0.068419, 0.0696),
            LevelBuilder::new(0.068417, 0.0034),
            LevelBuilder::new(0.068416, 0.0662),
        ],
        asks: vec![
            LevelBuilder::new(0.068427, 15.6307),
            LevelBuilder::new(0.068428, 5.4739),
            LevelBuilder::new(0.068429, 0.0034),
            LevelBuilder::new(0.06843, 0.2191),
            LevelBuilder::new(0.068431, 2.9373),
            LevelBuilder::new(0.068432, 4.7787),
            LevelBuilder::new(0.068434, 0.5048),
            LevelBuilder::new(0.068435, 0.7308),
            LevelBuilder::new(0.068437, 4.8445),
            LevelBuilder::new(0.068438, 4.0523),
        ],
    }
}

pub fn get_bitstamp_order_book_builder() -> OrderBookBuilder<Bitstamp> {
    OrderBookBuilder {
        bids: vec![
            LevelBuilder::new(0.06842268, 0.30600373),
            LevelBuilder::new(0.06842252, 1.13398078),
            LevelBuilder::new(0.0684214, 0.3),
            LevelBuilder::new(0.068419, 0.3),
            LevelBuilder::new(0.06841854, 0.71679187),
            LevelBuilder::new(0.06841583, 0.445),
            LevelBuilder::new(0.06841334, 0.3),
            LevelBuilder::new(0.06841052, 1.19423679),
            LevelBuilder::new(0.06840582, 0.445),
            LevelBuilder::new(0.06839848, 0.445),
            LevelBuilder::new(0.06839149, 1.13613266),
            LevelBuilder::new(0.06839148, 0.3),
            LevelBuilder::new(0.06839146, 0.3),
            LevelBuilder::new(0.06839145, 0.445),
            LevelBuilder::new(0.06839137, 0.3),
            LevelBuilder::new(0.06839006, 0.2),
            LevelBuilder::new(0.06838769, 0.3),
            LevelBuilder::new(0.06838756, 0.3),
            LevelBuilder::new(0.06838356, 2.38634194),
            LevelBuilder::new(0.06837876, 0.3),
            LevelBuilder::new(0.06837875, 0.445),
            LevelBuilder::new(0.06837189, 0.445),
            LevelBuilder::new(0.06836954, 3.58026541),
            LevelBuilder::new(0.0683635, 0.43),
            LevelBuilder::new(0.0683622, 0.445),
            LevelBuilder::new(0.06836157, 0.05349198),
            LevelBuilder::new(0.06835472, 5.81206582),
            LevelBuilder::new(0.06835471, 0.445),
            LevelBuilder::new(0.068341, 0.445),
            LevelBuilder::new(0.06833018, 0.445),
            LevelBuilder::new(0.06832632, 0.00297315),
            LevelBuilder::new(0.0683229, 1.11),
            LevelBuilder::new(0.06832049, 4.77573921),
            LevelBuilder::new(0.06831991, 0.445),
            LevelBuilder::new(0.06830868, 0.00297312),
            LevelBuilder::new(0.06830652, 0.445),
            LevelBuilder::new(0.06829811, 0.445),
            LevelBuilder::new(0.068293, 2.17),
            LevelBuilder::new(0.06829104, 0.00297309),
            LevelBuilder::new(0.06828463, 0.445),
            LevelBuilder::new(0.0682734, 0.00297306),
            LevelBuilder::new(0.06827196, 0.445),
            LevelBuilder::new(0.06826341, 0.445),
            LevelBuilder::new(0.06825596, 0.445),
            LevelBuilder::new(0.06825576, 0.00297303),
            LevelBuilder::new(0.06824735, 0.445),
            LevelBuilder::new(0.06823812, 0.002973),
            LevelBuilder::new(0.06823558, 0.445),
            LevelBuilder::new(0.06822673, 0.445),
            LevelBuilder::new(0.06822636, 0.00297298),
            LevelBuilder::new(0.06822048, 0.00297297),
            LevelBuilder::new(0.06821983, 0.445),
            LevelBuilder::new(0.06820622, 0.445),
            LevelBuilder::new(0.06820284, 0.00297294),
            LevelBuilder::new(0.06819408, 0.445),
            LevelBuilder::new(0.06819108, 0.00297292),
            LevelBuilder::new(0.06818691, 7.66),
            LevelBuilder::new(0.06818652, 0.445),
            LevelBuilder::new(0.0681852, 0.00297291),
            LevelBuilder::new(0.06818259, 4.48),
            LevelBuilder::new(0.06817702, 0.445),
            LevelBuilder::new(0.06817344, 0.00297289),
            LevelBuilder::new(0.06816756, 0.00297288),
            LevelBuilder::new(0.06816723, 0.445),
            LevelBuilder::new(0.0681597, 17.0),
            LevelBuilder::new(0.06815748, 0.445),
            LevelBuilder::new(0.0681558, 0.00297286),
            LevelBuilder::new(0.06814992, 0.00297285),
            LevelBuilder::new(0.06814987, 0.445),
            LevelBuilder::new(0.06814404, 0.00297284),
            LevelBuilder::new(0.06813816, 0.00297283),
            LevelBuilder::new(0.06813623, 0.445),
            LevelBuilder::new(0.06813228, 0.00297282),
            LevelBuilder::new(0.0681264, 0.00298281),
            LevelBuilder::new(0.06812052, 0.0029828),
            LevelBuilder::new(0.06811464, 0.00298279),
            LevelBuilder::new(0.06810876, 0.00298278),
            LevelBuilder::new(0.06810288, 0.00298277),
            LevelBuilder::new(0.0681, 14.0),
            LevelBuilder::new(0.068097, 0.00298276),
            LevelBuilder::new(0.06809112, 0.00298275),
            LevelBuilder::new(0.0680882, 3.31),
            LevelBuilder::new(0.06808524, 0.00298274),
            LevelBuilder::new(0.06807936, 0.00298273),
            LevelBuilder::new(0.06807349, 0.05003944),
            LevelBuilder::new(0.06807348, 0.00298272),
            LevelBuilder::new(0.0680728, 32.5),
            LevelBuilder::new(0.0680676, 0.00298271),
            LevelBuilder::new(0.06806172, 0.0029827),
            LevelBuilder::new(0.06805584, 0.00298269),
            LevelBuilder::new(0.06804996, 0.00298268),
            LevelBuilder::new(0.06804408, 0.00298267),
            LevelBuilder::new(0.0680382, 0.00298266),
            LevelBuilder::new(0.06803232, 0.00298265),
            LevelBuilder::new(0.06802644, 0.00298264),
            LevelBuilder::new(0.06802056, 0.00298263),
            LevelBuilder::new(0.06801468, 0.00298262),
            LevelBuilder::new(0.0680088, 0.00298261),
            LevelBuilder::new(0.06800292, 0.0029826),
            LevelBuilder::new(0.06799704, 0.00298259),
        ],
        asks: vec![
            LevelBuilder::new(0.06843007, 0.2),
            LevelBuilder::new(0.06844104, 0.2),
            LevelBuilder::new(0.06844191, 0.445),
            LevelBuilder::new(0.06844876, 0.445),
            LevelBuilder::new(0.06845472, 0.2),
            LevelBuilder::new(0.06845494, 0.7164166),
            LevelBuilder::new(0.06845579, 0.445),
            LevelBuilder::new(0.06846365, 0.445),
            LevelBuilder::new(0.06847073, 0.445),
            LevelBuilder::new(0.06847844, 1.44969516),
            LevelBuilder::new(0.06847845, 1.19304311),
            LevelBuilder::new(0.06848265, 0.445),
            LevelBuilder::new(0.06849082, 0.445),
            LevelBuilder::new(0.06849768, 0.445),
            LevelBuilder::new(0.06850507, 7.40247485),
            LevelBuilder::new(0.06850508, 0.445),
            LevelBuilder::new(0.06850844, 2.38905754),
            LevelBuilder::new(0.06851229, 0.445),
            LevelBuilder::new(0.0685123, 2.38544801),
            LevelBuilder::new(0.06851454, 3.57684693),
            LevelBuilder::new(0.0685173, 0.43),
            LevelBuilder::new(0.06851915, 0.445),
            LevelBuilder::new(0.06852738, 0.445),
            LevelBuilder::new(0.0685363, 0.445),
            LevelBuilder::new(0.0685454, 0.445),
            LevelBuilder::new(0.0685515, 1.11),
            LevelBuilder::new(0.06855579, 0.445),
            LevelBuilder::new(0.06856265, 0.445),
            LevelBuilder::new(0.06857202, 0.445),
            LevelBuilder::new(0.06857252, 4.76588892),
            LevelBuilder::new(0.06857894, 0.445),
            LevelBuilder::new(0.06858648, 0.445),
            LevelBuilder::new(0.06859403, 0.445),
            LevelBuilder::new(0.0685989, 2.17),
            LevelBuilder::new(0.06860543, 0.445),
            LevelBuilder::new(0.06861229, 0.445),
            LevelBuilder::new(0.06861403, 7.66),
            LevelBuilder::new(0.06862172, 0.445),
            LevelBuilder::new(0.0686579, 17.0),
            LevelBuilder::new(0.06868296, 0.00297318),
            LevelBuilder::new(0.068683, 0.00297301),
            LevelBuilder::new(0.0687006, 0.00296321),
            LevelBuilder::new(0.06870064, 0.00297304),
            LevelBuilder::new(0.06871824, 0.00296324),
            LevelBuilder::new(0.06871828, 0.00297307),
            LevelBuilder::new(0.06872448, 4.48),
            LevelBuilder::new(0.06873588, 0.00296327),
            LevelBuilder::new(0.06873592, 0.0029731),
            LevelBuilder::new(0.06875352, 0.0029633),
            LevelBuilder::new(0.06875356, 0.00297313),
            LevelBuilder::new(0.06877116, 0.00296333),
            LevelBuilder::new(0.0687712, 0.00297316),
            LevelBuilder::new(0.0687888, 0.00296336),
            LevelBuilder::new(0.06878884, 0.00297319),
            LevelBuilder::new(0.06880068, 0.00297287),
            LevelBuilder::new(0.06880644, 0.00296339),
            LevelBuilder::new(0.06880648, 0.00296322),
            LevelBuilder::new(0.06881832, 0.0029729),
            LevelBuilder::new(0.06882408, 0.00296342),
            LevelBuilder::new(0.06882412, 0.00296325),
            LevelBuilder::new(0.06883596, 0.00297293),
            LevelBuilder::new(0.06884172, 0.00296345),
            LevelBuilder::new(0.0688536, 0.00297296),
            LevelBuilder::new(0.06885936, 0.00296348),
            LevelBuilder::new(0.0688594, 0.00296331),
            LevelBuilder::new(0.06887124, 0.00297299),
            LevelBuilder::new(0.068877, 0.00296351),
            LevelBuilder::new(0.06887704, 0.00296334),
            LevelBuilder::new(0.06888888, 0.00297302),
            LevelBuilder::new(0.06889464, 0.00296354),
            LevelBuilder::new(0.06889468, 0.00296337),
            LevelBuilder::new(0.06890652, 0.00297305),
            LevelBuilder::new(0.06891228, 0.00296357),
            LevelBuilder::new(0.06891232, 0.0029634),
            LevelBuilder::new(0.06892416, 0.00297308),
            LevelBuilder::new(0.06892992, 0.0029536),
            LevelBuilder::new(0.06892996, 0.00296343),
            LevelBuilder::new(0.0689418, 0.00297311),
            LevelBuilder::new(0.06894756, 0.00295363),
            LevelBuilder::new(0.0689476, 0.00296346),
            LevelBuilder::new(0.06895306, 3.31),
            LevelBuilder::new(0.06895944, 0.00297314),
            LevelBuilder::new(0.0689652, 0.00295366),
            LevelBuilder::new(0.06896524, 0.00296349),
            LevelBuilder::new(0.06897708, 0.00297317),
            LevelBuilder::new(0.06898284, 0.00295369),
            LevelBuilder::new(0.06898288, 0.00296352),
            LevelBuilder::new(0.06899472, 0.0029732),
            LevelBuilder::new(0.06900048, 0.00295372),
            LevelBuilder::new(0.06900052, 0.00296355),
            LevelBuilder::new(0.06901236, 0.00296323),
            LevelBuilder::new(0.06901812, 0.00295375),
            LevelBuilder::new(0.06901816, 0.00296358),
            LevelBuilder::new(0.06903, 0.00296326),
            LevelBuilder::new(0.06903576, 0.00295378),
            LevelBuilder::new(0.0690358, 0.00295361),
            LevelBuilder::new(0.06904764, 0.00296328),
            LevelBuilder::new(0.0690534, 0.00295381),
            LevelBuilder::new(0.06905344, 0.00295364),
            LevelBuilder::new(0.06906528, 0.00296332),
        ],
    }
}