#[macro_use]
extern crate criterion;

use criterion::Criterion;

use sauron::{
    html::{attributes::*, *},
    vdom::diff,
    vdom::Node,
};

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const LOREM: &str = "
    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    Phasellus sed neque fringilla, hendrerit nisl vel, cursus nibh.
    Ut nec purus et mi hendrerit fermentum eget id ante.

    Nullam hendrerit ligula vitae augue tempus posuere.
    Ut porttitor felis quis lectus faucibus fermentum.
    Aenean vitae enim id dolor viverra bibendum ornare et massa.
    In varius odio nec mauris molestie, eu tincidunt libero finibus.

    Maecenas volutpat magna nec justo vehicula, vel rutrum purus varius.
    Vestibulum vel lectus in magna accumsan condimentum.
    Duis feugiat metus at dui commodo, at laoreet mauris iaculis.
    Sed molestie dui ac magna aliquam tincidunt.
    Vestibulum a quam ultrices, porta odio a, interdum nunc.

    Aliquam quis erat tristique, suscipit tortor non, sollicitudin metus.
    Morbi dignissim libero vel porta congue.
    Maecenas facilisis orci ut scelerisque euismod.
    Vivamus scelerisque neque iaculis imperdiet efficitur.
    Proin ac mi porta, rhoncus lacus eu, sollicitudin lorem.

    Proin id nunc venenatis, lacinia arcu mollis, vulputate libero.
    Suspendisse vitae tellus facilisis, dapibus felis quis, molestie odio.

    Ut non lorem sed nunc venenatis vestibulum.
    Nunc lobortis risus et est rutrum venenatis.

    Praesent vestibulum sem sit amet convallis vehicula.
    Nulla ac sapien ut justo porta lacinia in nec tortor.
    Sed sit amet purus nec nunc porta posuere ut nec erat.
    Sed nec erat vestibulum, condimentum justo id, vestibulum ex.

    Donec convallis urna ac libero elementum imperdiet.
    Nulla eget nibh sit amet ipsum pellentesque interdum.
    Aenean pharetra erat fermentum sapien egestas sagittis.
    In interdum turpis eu lorem ornare laoreet.

    Sed id sem blandit, dapibus magna non, sagittis turpis.
    In semper elit ac purus volutpat pulvinar.

    Etiam eget ipsum pharetra, sollicitudin urna mattis, laoreet ex.
    Nullam eget metus eu ex pretium consectetur.
    Integer eget sem quis magna dapibus congue ac sed justo.
    Etiam nec nunc sit amet justo maximus ullamcorper non quis tellus.
    Curabitur rutrum lectus tempus sapien consectetur dictum.

    Vestibulum at orci at odio vestibulum laoreet sit amet ac felis.
    Suspendisse pulvinar nulla mollis ex pulvinar, quis suscipit libero maximus.
    Integer ac sem volutpat, pretium augue sit amet, dignissim erat.

    Nulla a nisl sagittis, convallis sem eget, pellentesque libero.
    Suspendisse in risus eget metus egestas tempus.
    Pellentesque nec libero vel sem convallis feugiat placerat in urna.
    Nullam sodales tellus id neque rhoncus fermentum.
    Nulla et ipsum egestas, tincidunt risus eget, egestas dolor.

    Aliquam iaculis neque vitae ipsum vestibulum posuere.

    Vivamus vehicula velit et ligula pretium lacinia.
    Cras facilisis odio ut tristique tempus.
    Quisque consectetur augue non ultricies finibus.
    Sed quis magna porttitor, volutpat ante a, blandit nibh.
    Phasellus dapibus massa id felis tincidunt, ac fermentum augue accumsan.

    Integer faucibus ante a risus hendrerit, placerat dictum nunc varius.

    Aliquam blandit justo at turpis bibendum lacinia.
    Pellentesque non nisl a justo tempor dignissim.
    Vivamus commodo velit sagittis porta aliquet.
    Donec laoreet risus sed libero dignissim dictum.
    Fusce semper odio vel risus lacinia bibendum.

    Suspendisse in libero sodales felis molestie rutrum eu quis nibh.
    Donec nec dolor scelerisque, blandit erat nec, maximus mi.

    Duis tempus libero eu rhoncus cursus.
    Sed ultrices ligula id ligula accumsan varius.
    Nunc ultrices eros id feugiat congue.
    Donec condimentum quam eu leo pellentesque consequat.
    Praesent id nisi semper, malesuada est ut, dapibus sem.
    Suspendisse id magna tincidunt, posuere quam non, malesuada eros.

    Ut eleifend mauris ac sem tincidunt finibus.
    Fusce vitae sem id quam sollicitudin placerat.
    Donec cursus est in convallis venenatis.
    Phasellus ultricies diam ut pretium elementum.

    Nulla ut nunc viverra, euismod eros et, pellentesque dui.
    Integer vitae nibh blandit, dapibus eros non, aliquet tellus.
    Duis vitae quam sodales nibh viverra iaculis eget quis neque.

    Mauris tempor metus condimentum arcu luctus aliquet.
    Proin condimentum lectus et massa facilisis tincidunt.

    Morbi in sem vitae ex efficitur laoreet.
    Nullam rutrum arcu ut accumsan sagittis.
    Suspendisse tristique nibh ut egestas venenatis.

    Donec at lorem at nulla varius maximus.
    Sed vitae massa condimentum, semper metus maximus, dignissim ipsum.
    Praesent ut ex eget risus mattis placerat.
    Nunc vel tortor quis ex dignissim hendrerit.
    Quisque interdum magna nec hendrerit bibendum.

    In non leo a sem congue tempor.
    Morbi eget nunc eget lorem iaculis consectetur.
    Phasellus sed augue ac nisi cursus auctor eu at ex.
    Phasellus id elit non erat blandit tempor.
    Aliquam dictum ante a enim tempus porta.
    Proin fermentum arcu mollis magna vulputate varius efficitur non augue.

    Nunc imperdiet libero quis quam gravida, a aliquet orci maximus.
    Phasellus eu nisl efficitur, vulputate dui sit amet, commodo lectus.
    Nam molestie urna in efficitur sollicitudin.

    Duis tincidunt ipsum in tellus posuere posuere.
    Nam interdum justo a mi malesuada pulvinar.
    Integer sit amet erat vulputate, finibus augue nec, hendrerit dui.
    Sed in nisl non ex ultrices commodo.

    Nunc placerat augue a nisi posuere sodales.
    Donec a lacus gravida, viverra metus at, ullamcorper enim.
    Quisque nec sapien imperdiet, dapibus velit nec, sodales sem.
    Fusce varius urna in urna consequat imperdiet.
    In ultrices justo quis dolor condimentum varius eget in turpis.
    Nunc id nunc euismod, blandit dui eget, vehicula dui.

    Donec fringilla sem sit amet libero condimentum, at faucibus nunc porta.
    Nulla convallis purus non felis vehicula convallis.
    Ut vitae velit id purus ornare ornare.
    Phasellus nec mi eu sem facilisis dignissim quis quis elit.
    Quisque sit amet erat non quam pulvinar cursus.

    Ut ac sem sit amet elit semper convallis sit amet vitae orci.
    Donec finibus arcu id sollicitudin molestie.
    Vivamus tempor massa at lectus interdum bibendum.

    Etiam rutrum arcu eu ultricies semper.
    Integer gravida magna a enim lobortis, viverra blandit nisl tempor.
    Duis volutpat magna vitae sagittis gravida.

    Nulla vitae metus eget dolor tempor dictum.
    Ut quis massa eu ipsum rhoncus efficitur vel et velit.
    Praesent sit amet tellus id felis imperdiet consequat.
    Vestibulum a lectus sed velit tempus tincidunt.

    Nulla nec ex imperdiet, semper arcu vel, convallis purus.
    Cras id justo vel dui tristique suscipit efficitur vitae magna.

    Morbi fringilla magna a sagittis venenatis.
    Mauris interdum nulla eu nulla pretium, non blandit sem tincidunt.
    Praesent dignissim odio in interdum tincidunt.
    Morbi at purus et est imperdiet tincidunt sit amet sit amet libero.

    Aliquam mollis dolor sed placerat pretium.
    Mauris eu ex eget odio luctus dapibus vitae a orci.
    Quisque venenatis risus et ex tempus pulvinar.
    Quisque eu turpis id mauris fringilla iaculis.

    Aliquam pulvinar sapien sit amet mattis placerat.
    In ornare elit eu odio luctus, dapibus hendrerit metus auctor.
    Pellentesque malesuada leo id ligula pellentesque, eget vehicula urna mattis.
    Nulla malesuada ante in felis semper convallis.
    Aliquam sed eros sed erat elementum pellentesque.

    Aliquam dignissim arcu in urna vestibulum, eget condimentum lectus maximus.
    Praesent eget dolor ac quam congue efficitur.

    Duis ornare lectus quis enim molestie mattis.
    Sed mollis enim in lacus sollicitudin eleifend at at sapien.

    In et eros id purus varius eleifend eget vitae purus.
    In mollis libero quis justo tincidunt, sit amet mollis leo sollicitudin.
    Ut dignissim tortor nec eros lobortis, id lacinia nibh malesuada.
    Curabitur non mauris id tellus ullamcorper tristique.
    Cras blandit risus at urna rutrum, id euismod leo malesuada.

    Pellentesque rhoncus justo tempor urna sollicitudin mattis.
    Ut tincidunt nisi vitae dolor rutrum commodo.
    In at nunc tincidunt, fermentum arcu eget, tristique magna.
    Quisque ac eros viverra, congue ipsum in, efficitur sem.

    Pellentesque in nisl porta tellus volutpat rutrum ac ut nulla.
    Vestibulum eget velit sagittis, feugiat sapien eget, molestie metus.
    Donec mattis est convallis iaculis lacinia.
    In in quam tristique arcu laoreet elementum.
    Aenean a ipsum semper, faucibus turpis eget, elementum dui.

    Morbi rhoncus justo non lacus dictum, sed auctor lectus rutrum.
    Donec eleifend lorem nec facilisis viverra.
    Mauris vitae metus euismod, aliquet orci ut, interdum nisl.
    Aenean non dui consectetur, elementum risus ac, semper erat.
    Nulla efficitur risus a metus auctor fermentum.

    Nullam quis magna ut sem bibendum mollis sit amet ac diam.
    Etiam scelerisque justo nec dapibus sollicitudin.
    Proin sed enim vel erat aliquam viverra bibendum semper enim.

    Donec vitae dolor finibus, venenatis enim sit amet, mollis ipsum.
    Maecenas ultricies odio pulvinar faucibus egestas.
    Aenean consequat tellus ut neque molestie, non interdum metus volutpat.
    Etiam commodo magna quis porttitor blandit.
    In nec dui et ligula tempus egestas.

    Nam vitae eros vel urna sagittis ullamcorper.
    Morbi eleifend justo ut facilisis semper.
    Nunc malesuada dui vitae lorem aliquam, quis dapibus diam pretium.

    Nulla eget urna eget nisl lacinia tincidunt.
    Donec sit amet urna congue, tempor diam eu, finibus ex.
    Cras non dolor eget nisl posuere aliquet sit amet quis augue.
    Ut ac augue quis ipsum porttitor volutpat.

    Etiam a justo a ipsum vulputate porttitor in in dui.
    Phasellus et mauris bibendum, dapibus nisi nec, commodo odio.
    Etiam vitae orci faucibus, interdum mauris a, elementum magna.

    Sed quis diam nec risus dictum bibendum vitae vitae libero.
    Etiam in nisl id eros sollicitudin scelerisque eu non leo.
    Nulla at quam id quam dignissim facilisis eu nec nibh.
    Donec ut risus nec nibh porttitor elementum.

    Nunc eu arcu bibendum mi consectetur imperdiet at quis lectus.
    Vivamus eget arcu sed tellus luctus vestibulum eget ac sem.
    Vivamus dapibus mi id lacus bibendum, non porttitor enim ullamcorper.
    Sed eget felis sed risus aliquam eleifend.
    Pellentesque commodo lorem quis lacus vestibulum, eu lacinia turpis eleifend.

    Donec eleifend magna vel nisl pharetra eleifend.
    Integer rutrum risus id elit venenatis semper.
    Nulla efficitur lectus vitae tortor volutpat posuere.

    Nam egestas purus nec tortor finibus varius.
    Praesent imperdiet lacus id massa viverra vehicula.
    Nulla nec sapien id augue faucibus malesuada.
    Ut aliquam nunc vel malesuada venenatis.
    Mauris pharetra massa vel sem bibendum commodo.
    Cras lobortis augue eu nulla iaculis ornare.

    Nulla mattis leo tempus eros ultricies, ac vehicula purus vestibulum.

    Curabitur non urna et mauris faucibus lacinia.
    Maecenas sit amet dolor ut orci rutrum vehicula aliquet eget purus.
    Ut et enim commodo, efficitur velit eget, mollis massa.
    Ut quis quam eu magna condimentum cursus.
    Donec placerat felis vitae orci cursus, id finibus sem porttitor.
    Vestibulum vel orci at erat consectetur accumsan.

    Maecenas ac quam convallis, fringilla libero at, elementum nunc.

    Curabitur molestie lorem ac velit bibendum, et convallis lorem scelerisque.
    Etiam feugiat turpis nec dolor consectetur, non luctus nisl hendrerit.
    Nulla pulvinar erat sed maximus volutpat.
    Nunc lobortis metus quis egestas convallis.
    Morbi at magna at lectus pulvinar ultricies.

    Duis in ligula vitae ipsum eleifend egestas.
    In iaculis nibh vel dui tempus, ut sagittis orci gravida.
    Sed rutrum dolor ornare turpis finibus, eu tincidunt leo pulvinar.

    Vivamus et lorem sed mi malesuada faucibus.

    Mauris non diam sit amet ante ultrices feugiat sit amet vel sapien.
    In luctus elit eu diam consectetur, at convallis dui tempus.
    In quis sem vitae sapien suscipit laoreet.
    Suspendisse non lacus fermentum, tincidunt mi at, placerat nulla.
    Sed eget lorem sed nisi auctor dapibus.

    Duis eleifend odio convallis, accumsan velit sed, elementum nulla.
    Aliquam at purus tempus, maximus enim sed, tempor magna.

    Integer ornare lacus at sodales malesuada.

    Aenean ac nisi ut massa pellentesque tristique.
    Integer viverra magna pretium nisl cursus, at euismod odio rhoncus.
    Etiam imperdiet lectus id urna vulputate mattis.
    Mauris facilisis orci viverra, semper quam quis, pellentesque ligula.
    Praesent tincidunt eros venenatis eros finibus elementum.

    Vivamus rutrum enim eget auctor tempus.
    Vivamus eu orci a dolor vulputate sollicitudin vel iaculis ipsum.
    Aliquam auctor justo nec feugiat molestie.

    Donec sagittis mi eu efficitur auctor.
    Nulla ullamcorper nulla nec lorem hendrerit convallis.

    Nulla consectetur sem ullamcorper, eleifend magna sit amet, tincidunt erat.
    Morbi cursus nisi at tortor euismod condimentum pellentesque eget sapien.
    Etiam posuere ante at sem luctus ultricies.
    Ut ullamcorper velit et porttitor ullamcorper.
    Ut eget diam quis risus viverra volutpat quis id sem.

    Vivamus sit amet tellus tempus, faucibus lorem a, viverra tellus.
    Fusce tempor ex at ex consectetur, a sagittis arcu aliquam.

    Mauris elementum nulla imperdiet, vehicula augue et, efficitur libero.
    Nam gravida est et molestie blandit.
    Ut gravida massa elementum dui varius, ac sollicitudin ante aliquam.

    Nullam ultricies purus et felis mattis, vel fermentum nisl semper.
    Fusce nec urna eu sem sagittis condimentum a quis urna.
    Maecenas et metus maximus, aliquam elit tempor, eleifend lacus.

    Fusce in neque a sapien tempor commodo.
    Nulla ut sapien sit amet diam ullamcorper pellentesque fermentum in augue.
    Suspendisse nec lectus mollis, euismod ligula eget, dictum mauris.
    Sed pharetra sem sed nunc scelerisque, eu imperdiet odio molestie.
    Vestibulum scelerisque dolor eget massa pellentesque hendrerit.
    In et leo eget nisl ultrices tempor.

    Donec sit amet massa bibendum, venenatis ligula sit amet, semper ante.
    Sed non arcu fermentum, maximus quam sed, aliquet ante.
    Vestibulum tincidunt dolor vel finibus gravida.

    Proin mollis lectus sed tristique maximus.
    Ut nec magna mollis est gravida ullamcorper.
    Proin volutpat ligula sit amet odio tempor rutrum.
    Vivamus ullamcorper lectus at rutrum auctor.
    Duis finibus sem lacinia lorem facilisis imperdiet.
    Maecenas in tortor a tortor varius feugiat.

    Nam venenatis orci in sem tristique, in pretium mi aliquam.
    Aliquam dapibus diam at magna interdum laoreet.
    Phasellus ut odio quis lectus blandit lobortis vitae quis mauris.

    In viverra diam at nulla ultricies pretium.
    Donec a nunc pellentesque, fermentum augue quis, vulputate nisi.
    Vivamus luctus diam sit amet lorem varius tempor.
    Aliquam cursus libero porta dolor placerat pulvinar.
    Sed malesuada urna eu ligula blandit mollis.

    Pellentesque ac ligula eget libero tempor accumsan.
    Morbi eget ante non odio sodales faucibus.
    Integer laoreet odio vitae eros ultrices vestibulum.
    Sed tempor diam ut mauris aliquet, eu dapibus sem viverra.

    Etiam sed libero ac lacus sodales egestas.
    Donec iaculis diam ut viverra posuere.
    Donec eu nisi quis quam condimentum euismod non eget justo.
    Aenean interdum leo at nunc dignissim dapibus.
    Cras venenatis nunc sed mauris luctus imperdiet.

    Vestibulum hendrerit orci vel quam sodales, non interdum orci posuere.
    Suspendisse consectetur nibh nec placerat iaculis.
    Morbi at enim in nunc egestas vestibulum quis ac urna.

    Nunc tempus mi at purus interdum tincidunt.
    Nulla hendrerit felis eget odio tempor, nec cursus lorem blandit.
    Quisque eget libero nec ex tempus consectetur a non dolor.
    Suspendisse condimentum diam a tortor commodo imperdiet.
    Ut tincidunt dolor vel maximus condimentum.

    Mauris non erat euismod, eleifend risus vel, pulvinar risus.
    Phasellus tincidunt mi at metus bibendum sollicitudin.
    Proin eu sapien ultrices lectus ultricies dapibus.
    Mauris eu est efficitur, suscipit nibh id, ultrices ante.

    Quisque vulputate nibh id erat feugiat rutrum.

    Cras et lacus elementum, commodo dui in, dignissim magna.
    Suspendisse semper nulla ut ante bibendum maximus.
    Nulla id sem eget eros pellentesque finibus.
    Pellentesque ut nisl in turpis tempus elementum at vel sapien.
    Donec tristique nisl eu lorem posuere, ut tristique urna ornare.

    Mauris sit amet est vitae sem interdum tincidunt.
    In consectetur ante eu magna interdum lobortis.
    Fusce fermentum ex a velit tempor faucibus.
    Aliquam volutpat libero at lacus convallis maximus.
    Donec cursus odio in varius convallis.

    Suspendisse vel ante nec dui suscipit malesuada et eu purus.
    Curabitur faucibus libero fermentum vulputate euismod.
    Vestibulum placerat urna ac lacinia molestie.
    Morbi sed ligula hendrerit, rutrum lorem sed, pellentesque lorem.
    Quisque eu velit luctus, mattis diam sed, sodales tellus.

    Proin in velit finibus est feugiat consequat.
    Maecenas in turpis quis tortor aliquam finibus.

    Vestibulum a ipsum sit amet nisl tristique dignissim eu vel mauris.

    Fusce suscipit neque at finibus finibus.
    Sed at ipsum quis eros gravida ultricies in ut lectus.
    Maecenas vitae dolor sodales, suscipit tellus in, fringilla orci.
    Pellentesque at lorem a dui porttitor eleifend.
    Integer sit amet lectus a libero faucibus elementum id semper nunc.
    Mauris semper felis a egestas cursus.

    Cras id ex volutpat, molestie dui placerat, iaculis tellus.
    Etiam luctus dui in augue sodales, a lobortis neque tincidunt.
    Suspendisse tincidunt est nec lorem porta bibendum.
    Mauris vitae ligula a ipsum dignissim bibendum a dignissim nibh.
    Donec sit amet mauris ultrices, porttitor turpis tristique, mollis nulla.
    Donec vel leo euismod, commodo risus in, blandit ex.

    Sed sodales augue vitae est finibus sodales.

    Mauris lobortis ante at justo congue, nec ultrices metus dictum.
    Phasellus congue neque ut tortor pulvinar congue.
    Mauris consequat ipsum et consectetur vehicula.

    Proin posuere ipsum at fermentum mollis.
    Duis mollis ante at lacus tempor interdum.

    Duis condimentum orci ultricies, consequat ante at, faucibus odio.
    Nam ut justo volutpat, vestibulum enim vel, tincidunt enim.
    Morbi fermentum ipsum a lorem sodales commodo.
    Mauris venenatis mauris quis semper suscipit.
    Sed aliquet leo ac volutpat porttitor.

    Sed in ligula et dolor laoreet congue ullamcorper ac justo.

    Fusce vel ipsum eu lorem laoreet tristique quis at velit.
    Aenean quis urna ut dui bibendum tincidunt.
    Fusce et neque ac turpis sollicitudin rhoncus.

    Vestibulum et mauris nec quam vestibulum tristique vel vitae libero.
    Fusce nec velit quis lacus vestibulum vestibulum non sit amet dolor.
    Donec condimentum quam sed lectus sodales, vitae lacinia augue rutrum.
    Vivamus sed augue non mauris convallis sodales aliquet eu lacus.
    Cras imperdiet sem et orci auctor, in mattis est pulvinar.
    Donec mattis purus ut massa ultrices vulputate vel vel magna.

    Pellentesque luctus felis suscipit, laoreet erat eu, mollis felis.
    Maecenas fringilla quam ac nibh semper, ut rhoncus leo tincidunt.

    Donec rutrum ipsum in risus laoreet faucibus.
    Etiam eu tellus et mauris accumsan ornare.
    Nulla sit amet urna fermentum, congue dui sit amet, aliquet est.
    Donec at velit non sem lobortis luctus.
    Quisque et odio vitae nibh cursus mollis id sed justo.
    Donec vulputate nibh quis sem rutrum gravida id eget arcu.

    Donec feugiat lacus ut cursus viverra.

    Mauris luctus dui a tellus feugiat varius.
    Praesent vehicula justo ac rhoncus pellentesque.
    In pharetra tellus non efficitur porta.
    Vivamus rhoncus ex non sem ultrices lacinia.
    Praesent efficitur leo non purus ornare convallis.

    Fusce mollis est a sem venenatis, nec bibendum sem malesuada.
    Sed eget lorem vel leo posuere imperdiet.
    Donec efficitur nulla sit amet massa cursus, sit amet gravida justo tempus.
    Proin iaculis sapien viverra sem aliquam, et venenatis felis volutpat.
    Donec in tortor ut lacus tempor porttitor.

    Nam ultrices neque auctor, interdum lectus et, fringilla tortor.
    Sed bibendum urna rutrum metus tincidunt posuere.
    Quisque elementum leo nec lacus efficitur aliquet.
    Nulla iaculis massa non orci facilisis, vel sagittis erat auctor.

    Donec fermentum mauris vel massa vestibulum sagittis.
    Fusce id augue et nibh posuere molestie vitae ac est.

    Nunc efficitur neque non mauris commodo commodo.
    Nam aliquam neque et nulla vulputate tempus eu nec ipsum.
    Aliquam convallis turpis sit amet mauris mollis placerat.

    Quisque a dolor eget elit accumsan mattis vitae eget velit.
    Vestibulum quis leo varius, faucibus est a, commodo justo.
";

fn build_editor() {
    let _view: Node<()> = div(
        vec![class("code")],
        LOREM
            .lines()
            .enumerate()
            .map(|(n_line, line)| {
                let chars: Vec<char> = line.chars().collect();
                let mut hasher = DefaultHasher::new();
                chars.hash(&mut hasher);
                let line_hash = hasher.finish();

                div(
                    vec![class("line"), attr("n_line", n_line), key(line_hash)],
                    line.chars()
                        .enumerate()
                        .map(|(n_char, ch)| {
                            div(vec![class("ch"), attr("pos", n_char)], vec![text(ch)])
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
    );
}

fn build_100_child_nodes() {
    let _view: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| div(vec![class("child-div")], vec![text(format!("node: {}", n))]))
            .collect::<Vec<Node<()>>>(),
    );
}

fn diff_100() {
    let view1: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class(format!("child-div_{}", n))],
                    vec![text(format!("node: {}", n))],
                )
            })
            .collect::<Vec<Node<()>>>(),
    );

    let view2: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class(format!("child-div_{}", n + 1))],
                    vec![text(format!("node: {}", n))],
                )
            })
            .collect::<Vec<Node<()>>>(),
    );
    let node_diff = diff(&view1, &view2);
    assert_eq!(node_diff.len(), 100)
}

fn build_100_nodes_with_100_child_nodes() {
    let _view: Node<()> = div(
        vec![class("some-class")],
        (0..100)
            .into_iter()
            .map(|n| {
                div(
                    vec![class("parent"), class(n)],
                    (0..100)
                        .into_iter()
                        .map(|n2| {
                            div(
                                vec![class("child-div")],
                                vec![text(format!("node: {}", n2))],
                            )
                        })
                        .collect::<Vec<Node<()>>>(),
                )
            })
            .collect::<Vec<Node<()>>>(),
    );
}

fn bench1(c: &mut Criterion) {
    c.bench_function("100x100", |b| {
        b.iter(|| build_100_nodes_with_100_child_nodes())
    });
    c.bench_function("100", |b| b.iter(|| build_100_child_nodes()));
    c.bench_function("diff_100", |b| b.iter(|| diff_100()));
    c.bench_function("build_editor", |b| b.iter(|| build_editor()));
}

criterion_group!(benches, bench1);

criterion_main!(benches);
