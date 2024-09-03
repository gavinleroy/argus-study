(function() {var type_impls = {
"space":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FinderData-for-(F0,+F1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/debris.rs.html#108\">source</a><a href=\"#impl-FinderData-for-(F0,+F1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F0: <a class=\"trait\" href=\"space/prelude/trait.FinderData.html\" title=\"trait space::prelude::FinderData\">FinderData</a>, F1: <a class=\"trait\" href=\"space/prelude/trait.FinderData.html\" title=\"trait space::prelude::FinderData\">FinderData</a>&gt; <a class=\"trait\" href=\"space/prelude/trait.FinderData.html\" title=\"trait space::prelude::FinderData\">FinderData</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(F0, F1)</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.FinderData.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = (&lt;F0 as <a class=\"trait\" href=\"space/prelude/trait.FinderData.html\" title=\"trait space::prelude::FinderData\">FinderData</a>&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.FinderData.html#associatedtype.Item\" title=\"type space::prelude::FinderData::Item\">Item</a>, &lt;F1 as <a class=\"trait\" href=\"space/prelude/trait.FinderData.html\" title=\"trait space::prelude::FinderData\">FinderData</a>&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.FinderData.html#associatedtype.Item\" title=\"type space::prelude::FinderData::Item\">Item</a>)</h4></section></div></details>","FinderData","space::pos::Origin"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntoProbe%3C%3CF+as+ExclusiveProbeFunction%3CMarker%3E%3E::In,+%3CF+as+ExclusiveProbeFunction%3CMarker%3E%3E::Out,+(IsExclusiveProbeFunction,+Marker)%3E-for-F\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/probe.rs.html#92-98\">source</a><a href=\"#impl-IntoProbe%3C%3CF+as+ExclusiveProbeFunction%3CMarker%3E%3E::In,+%3CF+as+ExclusiveProbeFunction%3CMarker%3E%3E::Out,+(IsExclusiveProbeFunction,+Marker)%3E-for-F\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Marker, F&gt; <a class=\"trait\" href=\"space/prelude/trait.IntoProbe.html\" title=\"trait space::prelude::IntoProbe\">IntoProbe</a>&lt;&lt;F as <a class=\"trait\" href=\"space/prelude/trait.ExclusiveProbeFunction.html\" title=\"trait space::prelude::ExclusiveProbeFunction\">ExclusiveProbeFunction</a>&lt;Marker&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ExclusiveProbeFunction.html#associatedtype.In\" title=\"type space::prelude::ExclusiveProbeFunction::In\">In</a>, &lt;F as <a class=\"trait\" href=\"space/prelude/trait.ExclusiveProbeFunction.html\" title=\"trait space::prelude::ExclusiveProbeFunction\">ExclusiveProbeFunction</a>&lt;Marker&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ExclusiveProbeFunction.html#associatedtype.Out\" title=\"type space::prelude::ExclusiveProbeFunction::Out\">Out</a>, (<a class=\"struct\" href=\"space/prelude/struct.IsExclusiveProbeFunction.html\" title=\"struct space::prelude::IsExclusiveProbeFunction\">IsExclusiveProbeFunction</a>, Marker)&gt; for F<div class=\"where\">where\n    F: <a class=\"trait\" href=\"space/prelude/trait.ExclusiveProbeFunction.html\" title=\"trait space::prelude::ExclusiveProbeFunction\">ExclusiveProbeFunction</a>&lt;Marker&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Probe\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Probe\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.IntoProbe.html#associatedtype.Probe\" class=\"associatedtype\">Probe</a> = <a class=\"struct\" href=\"space/prelude/struct.ExclusiveFunctionProbe.html\" title=\"struct space::prelude::ExclusiveFunctionProbe\">ExclusiveFunctionProbe</a>&lt;Marker, F&gt;</h4></section></div></details>","IntoProbe<<F as ExclusiveProbeFunction<Marker>>::In, <F as ExclusiveProbeFunction<Marker>>::Out, (IsExclusiveProbeFunction, Marker)>","space::pos::Origin"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntoProbe%3C%3CF+as+ProbeFunction%3CMarker%3E%3E::In,+%3CF+as+ProbeFunction%3CMarker%3E%3E::Out,+(IsProbeFunction,+Marker)%3E-for-F\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/probe.rs.html#85-90\">source</a><a href=\"#impl-IntoProbe%3C%3CF+as+ProbeFunction%3CMarker%3E%3E::In,+%3CF+as+ProbeFunction%3CMarker%3E%3E::Out,+(IsProbeFunction,+Marker)%3E-for-F\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Marker, F&gt; <a class=\"trait\" href=\"space/prelude/trait.IntoProbe.html\" title=\"trait space::prelude::IntoProbe\">IntoProbe</a>&lt;&lt;F as <a class=\"trait\" href=\"space/prelude/trait.ProbeFunction.html\" title=\"trait space::prelude::ProbeFunction\">ProbeFunction</a>&lt;Marker&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ProbeFunction.html#associatedtype.In\" title=\"type space::prelude::ProbeFunction::In\">In</a>, &lt;F as <a class=\"trait\" href=\"space/prelude/trait.ProbeFunction.html\" title=\"trait space::prelude::ProbeFunction\">ProbeFunction</a>&lt;Marker&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ProbeFunction.html#associatedtype.Out\" title=\"type space::prelude::ProbeFunction::Out\">Out</a>, (<a class=\"struct\" href=\"space/prelude/struct.IsProbeFunction.html\" title=\"struct space::prelude::IsProbeFunction\">IsProbeFunction</a>, Marker)&gt; for F<div class=\"where\">where\n    F: <a class=\"trait\" href=\"space/prelude/trait.ProbeFunction.html\" title=\"trait space::prelude::ProbeFunction\">ProbeFunction</a>&lt;Marker&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Probe\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Probe\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.IntoProbe.html#associatedtype.Probe\" class=\"associatedtype\">Probe</a> = <a class=\"struct\" href=\"space/prelude/struct.FunctionProbe.html\" title=\"struct space::prelude::FunctionProbe\">FunctionProbe</a>&lt;Marker, F&gt;</h4></section></div></details>","IntoProbe<<F as ProbeFunction<Marker>>::In, <F as ProbeFunction<Marker>>::Out, (IsProbeFunction, Marker)>","space::pos::Origin"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IntoProbeConfigs%3C(SystemProbeTupleMarker,+P0,+P1)%3E-for-(S0,+S1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/probe.rs.html#24\">source</a><a href=\"#impl-IntoProbeConfigs%3C(SystemProbeTupleMarker,+P0,+P1)%3E-for-(S0,+S1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P0, S0, P1, S1&gt; <a class=\"trait\" href=\"space/prelude/trait.IntoProbeConfigs.html\" title=\"trait space::prelude::IntoProbeConfigs\">IntoProbeConfigs</a>&lt;(<a class=\"struct\" href=\"space/prelude/struct.SystemProbeTupleMarker.html\" title=\"struct space::prelude::SystemProbeTupleMarker\">SystemProbeTupleMarker</a>, P0, P1)&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(S0, S1)</a><div class=\"where\">where\n    S0: <a class=\"trait\" href=\"space/prelude/trait.IntoProbeConfigs.html\" title=\"trait space::prelude::IntoProbeConfigs\">IntoProbeConfigs</a>&lt;P0&gt;,\n    S1: <a class=\"trait\" href=\"space/prelude/trait.IntoProbeConfigs.html\" title=\"trait space::prelude::IntoProbeConfigs\">IntoProbeConfigs</a>&lt;P1&gt;,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.IntoProbeConfigs.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = (&lt;S0 as <a class=\"trait\" href=\"space/prelude/trait.IntoProbeConfigs.html\" title=\"trait space::prelude::IntoProbeConfigs\">IntoProbeConfigs</a>&lt;P0&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.IntoProbeConfigs.html#associatedtype.Item\" title=\"type space::prelude::IntoProbeConfigs::Item\">Item</a>, &lt;S1 as <a class=\"trait\" href=\"space/prelude/trait.IntoProbeConfigs.html\" title=\"trait space::prelude::IntoProbeConfigs\">IntoProbeConfigs</a>&lt;P1&gt;&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.IntoProbeConfigs.html#associatedtype.Item\" title=\"type space::prelude::IntoProbeConfigs::Item\">Item</a>)</h4></section></div></details>","IntoProbeConfigs<(SystemProbeTupleMarker, P0, P1)>","space::pos::Origin"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Pos-for-(X,+Y)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/pos.rs.html#39-42\">source</a><a href=\"#impl-Pos-for-(X,+Y)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;X: <a class=\"trait\" href=\"space/prelude/trait.Num.html\" title=\"trait space::prelude::Num\">Num</a>, Y: <a class=\"trait\" href=\"space/prelude/trait.Num.html\" title=\"trait space::prelude::Num\">Num</a>&gt; <a class=\"trait\" href=\"space/prelude/trait.Pos.html\" title=\"trait space::prelude::Pos\">Pos</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(X, Y)</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.X\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.X\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.Pos.html#associatedtype.X\" class=\"associatedtype\">X</a> = X</h4></section><section id=\"associatedtype.Y\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Y\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.Pos.html#associatedtype.Y\" class=\"associatedtype\">Y</a> = Y</h4></section></div></details>","Pos","space::pos::Origin"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ProbeParam-for-(P0,+P1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/probe.rs.html#66\">source</a><a href=\"#impl-ProbeParam-for-(P0,+P1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P0: <a class=\"trait\" href=\"space/prelude/trait.ProbeParam.html\" title=\"trait space::prelude::ProbeParam\">ProbeParam</a>, P1: <a class=\"trait\" href=\"space/prelude/trait.ProbeParam.html\" title=\"trait space::prelude::ProbeParam\">ProbeParam</a>&gt; <a class=\"trait\" href=\"space/prelude/trait.ProbeParam.html\" title=\"trait space::prelude::ProbeParam\">ProbeParam</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(P0, P1)</a></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"space/prelude/trait.ProbeParam.html#associatedtype.Item\" class=\"associatedtype\">Item</a> = (&lt;P0 as <a class=\"trait\" href=\"space/prelude/trait.ProbeParam.html\" title=\"trait space::prelude::ProbeParam\">ProbeParam</a>&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ProbeParam.html#associatedtype.Item\" title=\"type space::prelude::ProbeParam::Item\">Item</a>, &lt;P1 as <a class=\"trait\" href=\"space/prelude/trait.ProbeParam.html\" title=\"trait space::prelude::ProbeParam\">ProbeParam</a>&gt;::<a class=\"associatedtype\" href=\"space/prelude/trait.ProbeParam.html#associatedtype.Item\" title=\"type space::prelude::ProbeParam::Item\">Item</a>)</h4></section></div></details>","ProbeParam","space::pos::Origin"],["<section id=\"impl-Debris-for-(F0,+F1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/debris.rs.html#110\">source</a><a href=\"#impl-Debris-for-(F0,+F1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F0: <a class=\"trait\" href=\"space/prelude/trait.Debris.html\" title=\"trait space::prelude::Debris\">Debris</a>, F1: <a class=\"trait\" href=\"space/prelude/trait.Debris.html\" title=\"trait space::prelude::Debris\">Debris</a>&gt; <a class=\"trait\" href=\"space/prelude/trait.Debris.html\" title=\"trait space::prelude::Debris\">Debris</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(F0, F1)</a></h3></section>","Debris","space::pos::Origin"],["<section id=\"impl-Rock-for-(F0,+F1)\" class=\"impl\"><a class=\"src rightside\" href=\"src/space/debris.rs.html#109\">source</a><a href=\"#impl-Rock-for-(F0,+F1)\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F0: <a class=\"trait\" href=\"space/prelude/trait.Rock.html\" title=\"trait space::prelude::Rock\">Rock</a>, F1: <a class=\"trait\" href=\"space/prelude/trait.Rock.html\" title=\"trait space::prelude::Rock\">Rock</a>&gt; <a class=\"trait\" href=\"space/prelude/trait.Rock.html\" title=\"trait space::prelude::Rock\">Rock</a> for <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.80.1/std/primitive.tuple.html\">(F0, F1)</a></h3></section>","Rock","space::pos::Origin"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()