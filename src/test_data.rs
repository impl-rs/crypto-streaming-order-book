use crate::{
    binance::Binance,
    bitstamp::Bitstamp,
    order_book::{LevelBuilder, OrderBookBuilder},
};

pub fn get_binance_order_book_builder() -> OrderBookBuilder<Binance> {
    OrderBookBuilder {
        bids: vec![
            LevelBuilder::new(0.068426, 23.8545),
            LevelBuilder::new(0.068425, 4.7825),
            LevelBuilder::new(0.068424, 1.0957),
            LevelBuilder::new(0.06842268, 0.4643),
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

pub fn get_binance_websocket_response() -> &'static str {
    "{\"lastUpdateId\":6630723519,\"bids\":[[\"0.06795500\",\"20.71540000\"],[\"0.06795400\",\"0.20000000\"],[\"0.06795300\",\"1.30030000\"],[\"0.06795100\",\"1.53340000\"],[\"0.06795000\",\"2.10520000\"],[\"0.06794900\",\"0.69620000\"],[\"0.06794800\",\"0.64150000\"],[\"0.06794700\",\"0.18030000\"],[\"0.06794600\",\"1.09700000\"],[\"0.06794500\",\"4.49010000\"]],\"asks\":[[\"0.06795600\",\"13.99720000\"],[\"0.06795700\",\"3.72720000\"],[\"0.06795800\",\"5.28970000\"],[\"0.06795900\",\"0.02970000\"],[\"0.06796000\",\"1.76030000\"],[\"0.06796100\",\"0.04260000\"],[\"0.06796300\",\"0.06960000\"],[\"0.06796400\",\"0.60260000\"],[\"0.06796500\",\"1.00340000\"],[\"0.06796600\",\"0.06620000\"]]}"
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

pub fn get_bitstamp_websocket_response() -> &'static str {
    "{\"data\":{\"timestamp\":\"1682167286\",\"microtimestamp\":\"1682167286795358\",\"bids\":[[\"0.06791795\",\"0.44453153\"],[\"0.06790535\",\"0.40000000\"],[\"0.06790534\",\"0.55000000\"],[\"0.06790467\",\"0.40000000\"],[\"0.06790466\",\"0.40000000\"],[\"0.06789815\",\"0.80841402\"],[\"0.06789754\",\"0.40000000\"],[\"0.06789753\",\"0.55000000\"],[\"0.06789065\",\"0.40000000\"],[\"0.06789064\",\"0.55000000\"],[\"0.06788955\",\"1.34755666\"],[\"0.06788578\",\"0.40000000\"],[\"0.06788330\",\"0.55000000\"],[\"0.06787416\",\"6.53690684\"],[\"0.06787415\",\"0.55000000\"],[\"0.06787120\",\"1.13580060\"],[\"0.06786890\",\"0.45586860\"],[\"0.06786563\",\"0.55000000\"],[\"0.06786467\",\"2.69612070\"],[\"0.06785605\",\"4.04522114\"],[\"0.06785600\",\"0.55000000\"],[\"0.06785000\",\"6.18100000\"],[\"0.06784307\",\"0.55000000\"],[\"0.06784209\",\"4.19878778\"],[\"0.06783608\",\"0.55000000\"],[\"0.06783000\",\"19.41630000\"],[\"0.06782702\",\"0.55000000\"],[\"0.06781886\",\"0.55000000\"],[\"0.06781172\",\"0.55000000\"],[\"0.06780153\",\"0.55000000\"],[\"0.06780000\",\"36.81900000\"],[\"0.06779345\",\"5.40083113\"],[\"0.06779298\",\"0.55000000\"],[\"0.06778367\",\"0.55000000\"],[\"0.06777600\",\"0.55000000\"],[\"0.06776730\",\"2.29751940\"],[\"0.06776651\",\"0.55000000\"],[\"0.06775508\",\"0.55000000\"],[\"0.06774828\",\"0.55000000\"],[\"0.06774420\",\"0.00299216\"],[\"0.06774150\",\"0.55000000\"],[\"0.06773468\",\"0.55000000\"],[\"0.06772656\",\"0.00299213\"],[\"0.06772419\",\"0.55000000\"],[\"0.06771495\",\"0.55000000\"],[\"0.06770892\",\"0.00299210\"],[\"0.06770537\",\"0.55000000\"],[\"0.06769402\",\"0.55000000\"],[\"0.06769300\",\"19.20000000\"],[\"0.06769128\",\"0.00299207\"],[\"0.06768509\",\"0.55000000\"],[\"0.06767952\",\"0.00299205\"],[\"0.06767364\",\"0.00299204\"],[\"0.06766188\",\"0.00300202\"],[\"0.06765600\",\"0.00300201\"],[\"0.06764903\",\"8.00647980\"],[\"0.06764424\",\"0.00300199\"],[\"0.06763836\",\"0.00300198\"],[\"0.06762660\",\"0.00300196\"],[\"0.06762072\",\"0.00300195\"],[\"0.06761161\",\"0.02553348\"],[\"0.06761160\",\"36.20000000\"],[\"0.06760896\",\"0.00300193\"],[\"0.06760308\",\"0.00300192\"],[\"0.06760000\",\"3.00000000\"],[\"0.06759132\",\"0.00300190\"],[\"0.06758544\",\"0.00300189\"],[\"0.06757368\",\"0.00300187\"],[\"0.06756780\",\"0.00300186\"],[\"0.06755604\",\"0.00300184\"],[\"0.06755016\",\"0.00300183\"],[\"0.06753840\",\"0.00300181\"],[\"0.06753252\",\"0.00300180\"],[\"0.06752595\",\"4.49720040\"],[\"0.06752076\",\"0.00300178\"],[\"0.06751488\",\"0.00300177\"],[\"0.06750312\",\"0.00300175\"],[\"0.06749724\",\"0.00300174\"],[\"0.06748548\",\"0.00300172\"],[\"0.06747960\",\"0.00300171\"],[\"0.06747372\",\"0.00300170\"],[\"0.06746784\",\"0.00300169\"],[\"0.06746620\",\"38.10000000\"],[\"0.06746196\",\"0.00300168\"],[\"0.06745608\",\"0.00300167\"],[\"0.06745020\",\"0.00300166\"],[\"0.06744432\",\"0.00301165\"],[\"0.06743844\",\"0.00301164\"],[\"0.06743256\",\"0.00301163\"],[\"0.06742668\",\"0.00301162\"],[\"0.06742080\",\"0.00301161\"],[\"0.06741492\",\"0.00301160\"],[\"0.06740904\",\"0.00301159\"],[\"0.06740316\",\"0.00301158\"],[\"0.06740068\",\"3.46529700\"],[\"0.06739728\",\"0.00301157\"],[\"0.06739140\",\"0.00301156\"],[\"0.06738552\",\"0.00301155\"],[\"0.06737964\",\"0.00301154\"],[\"0.06737376\",\"0.00301153\"]],\"asks\":[[\"0.06792853\",\"0.55000000\"],[\"0.06792858\",\"0.40000000\"],[\"0.06793198\",\"0.20000000\"],[\"0.06793538\",\"0.55000000\"],[\"0.06793705\",\"0.80795120\"],[\"0.06794253\",\"0.40000000\"],[\"0.06794333\",\"0.40000000\"],[\"0.06794334\",\"0.55000000\"],[\"0.06794520\",\"1.34647060\"],[\"0.06795205\",\"0.55000000\"],[\"0.06795885\",\"0.55000000\"],[\"0.06796665\",\"0.55000000\"],[\"0.06796730\",\"0.40000000\"],[\"0.06797511\",\"0.55000000\"],[\"0.06797718\",\"2.69199156\"],[\"0.06798361\",\"0.55000000\"],[\"0.06798792\",\"0.29670000\"],[\"0.06799249\",\"0.55000000\"],[\"0.06799996\",\"0.55000000\"],[\"0.06800160\",\"0.45586860\"],[\"0.06800802\",\"0.55000000\"],[\"0.06800967\",\"4.03479653\"],[\"0.06801000\",\"5.65890000\"],[\"0.06802079\",\"0.55000000\"],[\"0.06802758\",\"0.55000000\"],[\"0.06803437\",\"0.55000000\"],[\"0.06804039\",\"5.38013230\"],[\"0.06804100\",\"1.13580060\"],[\"0.06804399\",\"0.55000000\"],[\"0.06805000\",\"16.42850000\"],[\"0.06805739\",\"0.55000000\"],[\"0.06807085\",\"8.55549130\"],[\"0.06807086\",\"0.55000000\"],[\"0.06807823\",\"0.55000000\"],[\"0.06808000\",\"33.14500000\"],[\"0.06808550\",\"2.29751940\"],[\"0.06808712\",\"0.55000000\"],[\"0.06809769\",\"0.55000000\"],[\"0.06810084\",\"0.00299219\"],[\"0.06810457\",\"0.55000000\"],[\"0.06810942\",\"8.00647980\"],[\"0.06811163\",\"0.55000000\"],[\"0.06811848\",\"0.00299222\"],[\"0.06812385\",\"0.55000000\"],[\"0.06813036\",\"0.00300173\"],[\"0.06813065\",\"0.55000000\"],[\"0.06813612\",\"0.00299225\"],[\"0.06813616\",\"0.00299208\"],[\"0.06813808\",\"0.55000000\"],[\"0.06814711\",\"0.55000000\"],[\"0.06814800\",\"0.00300176\"],[\"0.06815376\",\"0.00299228\"],[\"0.06815380\",\"0.00299211\"],[\"0.06815391\",\"0.55000000\"],[\"0.06816254\",\"0.55000000\"],[\"0.06816564\",\"0.00300179\"],[\"0.06817000\",\"0.01900000\"],[\"0.06817140\",\"0.00598462\"],[\"0.06817144\",\"0.00299214\"],[\"0.06818328\",\"0.00300182\"],[\"0.06818737\",\"4.49720040\"],[\"0.06818904\",\"0.00299234\"],[\"0.06818908\",\"0.00299217\"],[\"0.06820092\",\"0.00300185\"],[\"0.06820668\",\"0.00299237\"],[\"0.06820672\",\"0.00299220\"],[\"0.06821856\",\"0.00300188\"],[\"0.06822432\",\"0.00299240\"],[\"0.06822436\",\"0.00299223\"],[\"0.06823620\",\"0.00300191\"],[\"0.06824196\",\"0.00298243\"],[\"0.06824200\",\"0.00299226\"],[\"0.06825384\",\"0.00300194\"],[\"0.06825960\",\"0.00298246\"],[\"0.06825964\",\"0.00299229\"],[\"0.06827148\",\"0.00300197\"],[\"0.06827724\",\"0.00298249\"],[\"0.06827728\",\"0.00598464\"],[\"0.06828912\",\"0.00300200\"],[\"0.06829488\",\"0.00298252\"],[\"0.06829492\",\"0.02934863\"],[\"0.06830300\",\"0.23009231\"],[\"0.06830676\",\"0.00300203\"],[\"0.06831252\",\"0.00298255\"],[\"0.06831256\",\"0.00299238\"],[\"0.06832440\",\"0.00299206\"],[\"0.06833016\",\"0.00298258\"],[\"0.06833020\",\"0.00299241\"],[\"0.06834204\",\"0.00299209\"],[\"0.06834780\",\"0.00298261\"],[\"0.06834784\",\"0.00298244\"],[\"0.06835968\",\"0.00299212\"],[\"0.06836544\",\"0.00298264\"],[\"0.06836548\",\"0.00298247\"],[\"0.06837149\",\"3.46529700\"],[\"0.06837732\",\"0.00299215\"],[\"0.06838308\",\"0.00298267\"],[\"0.06838312\",\"0.00298250\"],[\"0.06839496\",\"0.00299218\"],[\"0.06840072\",\"0.00298270\"]]},\"channel\":\"order_book_ethbtc\",\"event\":\"data\"}"
}
