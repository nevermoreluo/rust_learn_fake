use crate::locales::Data;


#[derive(Copy, Clone)]
#[allow(non_camel_case_types)] 
pub struct ZH_CN;

impl Data for ZH_CN {
    const CATS: &'static [&'static str] = &["中华田园猫","中国狸花猫","山东狮子猫","玄猫","黑白花猫","三花猫","玳瑁猫","橘猫","四川简州猫","中国大白猫","美国短毛猫","英国短毛猫","加菲猫","波斯猫","布偶猫","苏格兰折耳猫","暹罗猫","斯芬克斯猫","德文卷毛猫","阿比西尼亚猫"];
    const DOGS:  &'static [&'static str] = &["藏獒","袖狗","拉萨狮子犬","西藏狮子犬","松狮犬","中国冠毛犬","西施犬","沙皮犬","八哥犬","西藏獚","中华田园犬","下司犬","北京犬","西藏梗","柴犬","哈士奇","德国牧羊犬","边境牧羊犬","贵兵犬","秋田犬","罗威纳犬","蝴蝶犬","英国斗牛犬","阿富汗猎犬","萨摩耶犬","大白熊犬","比利时牧羊犬","美国爱斯基摩犬","彭布罗克威尔士柯基犬","墨西哥无毛犬"];
}
