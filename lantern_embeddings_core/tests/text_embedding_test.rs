use lantern_embeddings_core::core::{get_runtime, Runtime};

static HELLO_WORLD_TEXT: &'static str = "Hello world!";
#[rustfmt::skip]
static HELLO_WORLD_CLIP_EMB: &'static [f32] = &[-0.020476775, 0.111073, -0.10530874, 0.19828044, -0.027562656, -0.23474982, 0.21065629, -1.4416628, 0.14605063, -0.160044, -0.37608588, -0.2119374, -0.15697962, 0.008458294, 0.2106666, -0.017877355, 0.33919975, 0.045852974, -0.100722946, -0.045923192, 0.10990898, -0.25829625, 0.30317736, 0.11488654, -0.46186274, -0.4626967, -0.042635046, 0.29741955, -0.037274115, 0.27366883, 0.20929608, -0.16471854, 0.055385463, -0.24945396, 0.046744138, 0.23008388, -0.187143, -0.029809874, 0.07640592, -0.16700715, -0.085975975, -0.25384986, -0.11262718, -0.13075642, 0.14013499, -0.08889471, 0.063620545, -0.03663072, 0.07881963, 0.229372, 0.29210842, -0.31412473, -0.0078749135, -0.28392997, 0.15476987, -0.02043429, -0.06290087, -0.018591419, -0.08700596, -0.19504404, 0.44238496, -0.109749734, 0.275388, -0.05582391, 0.13863862, -0.033968866, 0.19097951, -0.13578185, 0.14519869, 0.045676023, 0.25584215, -0.015645579, -0.26207292, 0.24827223, 0.03445548, -0.31498873, 0.2824291, 0.56327266, 0.013674453, -0.16007274, -0.21102041, -0.19478007, -0.14121902, -0.16792865, -0.40819037, 0.0438246, 0.22567344, -0.3297562, 0.11681607, 0.11226921, 0.003411904, 0.0825413, -1.9214885, -0.20295206, -0.01407405, -0.12056728, -0.10185087, 0.2004847, -0.10759242, -0.19343027, 0.23292445, 0.33320382, 0.22149219, -0.08638317, 0.23560561, -0.19066495, 0.17755258, -0.16993555, -0.029849865, -0.22932798, 0.08929604, 0.03428103, -0.107722655, -0.024028901, 0.1847432, 0.09379715, 0.068997346, 0.14411347, 0.07746827, 0.04011213, 0.0658475, 0.20222189, -0.030440576, 0.08776434, -0.13246597, 0.016684677, 0.047582008, 0.05198705, 0.0700707, 0.084653914, 0.06909941, 0.10376939, -0.081172146, 7.3704977, -0.14797403, 0.21361199, -0.08800018, -0.35132965, 0.021939317, -0.09089277, 0.037140936, -0.35118827, -0.04941287, -0.026789397, -0.23948334, 0.10874768, -0.26285917, 0.36523858, 0.50445855, 0.10481266, 0.4098543, -0.3129306, -0.59758216, -0.034414284, 0.027843453, -0.091017574, -0.32560846, 0.30377156, -0.23118782, 0.065180466, -0.36710367, 0.012223348, -0.16262382, 0.13480008, 0.13736263, 0.0075045526, 0.23728539, 0.2116946, 0.3838804, 0.14245796, 0.029927406, -0.1341641, -0.03992462, -0.11786532, -0.30819097, -0.12132531, -0.30330336, -0.12580931, -0.035588562, 0.16817878, -0.038514666, 0.18975018, 0.024216771, 0.026217014, -0.090409905, -0.07525094, 0.27214733, 0.044363648, 0.22049513, -0.16281407, 0.0631836, -0.053991392, 0.22901943, 0.060611814, 0.32349586, 0.012564644, -0.08271384, -0.078071356, -0.040050197, 0.30019575, -0.020119771, -0.24983557, -0.27131745, -0.054717228, 0.10751669, -0.22137052, -0.013891213, 0.1374619, 0.010636002, 0.14240754, 0.15695044, 0.13009356, 0.09398372, -0.025530757, -0.23091945, 0.025572032, 0.0031829178, -0.24976723, 0.21949057, -0.27335533, 0.10732825, -0.06507754, -0.13964619, -0.12820473, 0.0877506, -0.12037073, -0.09331375, 0.5416604, 0.09421308, 0.10336839, 0.13041648, 0.028918259, -0.20625801, 0.43129992, 0.1459057, -0.016492277, 0.04081285, 0.047613487, 0.009934774, 0.009535242, 0.24991985, 0.30162302, -0.035831526, -0.486154, -0.05103448, 0.6563518, 0.009447038, -0.46683514, -0.08313032, 0.03696554, 0.14653865, -0.18361577, -0.050656207, -0.22145961, -0.24882938, 0.16182947, 0.038483176, 0.021601643, 0.21019635, -0.08234185, 0.2780598, -0.2908701, 0.02953285, 0.12296721, -0.21878204, -0.128144, -0.060083054, -0.24550691, -0.0879209, 0.07546242, -0.07249651, 0.020485193, -0.03692793, 0.16765073, -0.09103814, 0.23033753, -0.21836475, 0.1664663, -0.21737409, 0.16596827, 0.31133112, -0.10023908, 0.3706924, -0.2464955, 0.24176213, 0.32084322, 0.16259453, -0.05540514, 0.05083946, -0.12935856, 0.03267494, 0.18455097, 0.011275742, 0.101560265, -0.03561543, 0.27086422, -0.09041187, -0.3287123, 0.040132664, -0.1831892, 0.0047493726, -0.033484608, 0.19651815, -0.31473777, 0.13112763, 0.3413608, -0.2455968, 0.045542836, -0.104461126, 0.005065279, 0.4251323, -0.23926145, 7.362257, -0.23300058, -0.007646084, 0.045260623, -0.04437712, -0.15608308, 0.10616787, 0.07287562, 0.020465162, 0.5783111, -0.09140764, 0.13351701, -0.10439435, -0.25910264, -0.24776801, -0.11816393, -0.0561099, -2.770246, -0.31068143, 0.14091884, 0.070470706, -0.058813483, -0.024911582, -0.15688656, 0.21985042, 0.36713743, 0.107128955, 0.13669589, 0.0051332787, 0.23876746, 0.40614, 0.060822293, -0.33256605, 0.08021091, -0.07582166, -0.23415035, -0.04432006, -0.07010134, -0.22593829, -0.065982476, 0.0427081, 0.028756332, -0.060415134, 0.4012118, 0.04000116, 0.38686636, 0.15789485, -0.3262711, 0.00475822, 0.1094888, 0.57392967, 0.34066314, -0.20665756, -0.057930544, 0.22794127, 0.15457465, -0.18280812, 0.09717287, -0.059893936, 0.6086165, -0.21629295, 0.23789874, -0.29008394, 0.20201065, -0.32359564, 0.10615185, -0.27292472, -0.07158633, -0.28011638, -0.34555736, 0.06371697, -0.08473098, 0.06386079, 0.056933045, -0.22559868, -0.14229044, -0.28497994, 0.12396638, -0.22131008, -0.5294925, -0.0767176, 0.11793665, 0.057084702, 0.0040413737, -0.06450936, 0.01728496, -0.3512309, 0.3330403, -0.06519018, 0.20660913, 0.05178895, 0.343586, 0.20610496, 0.15943015, -0.20400554, 0.030726567, 0.30282435, -0.010365292, 0.09665407, 0.14873841, 0.25734085, 0.21442415, 0.40125173, -0.08738096, -0.06913735, 0.2987156, 0.27425864, -0.00093972683, 0.04379954, 0.038295284, 0.06546015, -0.32040307, -0.20215109, 0.41629654, 0.047880255, -0.21194798, -0.036038153, 0.1522459, -0.19484445, 0.039462823, -0.21064907, -0.13694683, -0.2501285, 0.082520045, -0.19627452, -0.4384591, 0.23384215, 0.109192975, 0.15369438, -0.27861565, 0.11628551, -0.040622413, -0.04433043, -0.12166405, 0.13092265, -0.07827225, 0.11547213, -0.030894607, -0.18117768, 0.020442963, -0.17173964, 0.2974965, 0.32407364, 0.04766877, 0.10717582, 0.09193277, -0.16506508, 0.13757893, 0.096764445, -0.5020385, -0.016144186, 0.019135457, -0.29343155, -0.100692585, 0.37422845, 0.022318047, -0.15931854, 0.017905466, -0.13255252, 0.109108776, 0.0065737963, -0.074496314, 0.16018367, 0.16922836, 0.10337954, 0.14559221, 0.12458767, 0.22240283, -0.028031524, -0.80666745, -0.12128662, 0.01321203, -0.3255096, -0.06868529, -0.19293119, -0.025735162, -0.28654903, 0.020253882, -0.38534135, -0.038486935, 0.06613082, -0.06418133, -0.15839031, 0.06471726, -0.19283788, -0.34131792, 0.15264772, -0.23320137, -0.089332774, -0.041326385, 0.13021827, 0.102788255, -0.022895776, 0.09561402, 0.10994218, -0.032236956, -0.024895646, -0.2531594, -0.27954918, 0.23604941];
#[rustfmt::skip]
static HELLO_WORLD_BGE_SMALL_EMB: &'static [f32] = &[-0.030880496, -0.11021069, 0.3917847, -0.35962796, 0.22797719, 0.1286265, 0.104809806, 0.4536326, 0.20183088, 0.1336294, -0.028951608, -0.3623364, 0.088983126, 0.46398085, 0.3589503, 0.21120659, 0.33224252, -0.12301457, -1.0269446, -0.19482881, 1.0011039, 0.4460324, -0.19272235, -0.47009823, 0.009932369, -0.14311227, -0.13751373, 0.3344442, -0.014674753, -1.6587354, -0.13096964, -0.11285492, 0.91281325, -0.099221066, 0.35517678, -0.17221186, -0.023663897, 0.45018157, -0.42092425, 0.07768755, 0.39460424, -0.11940934, -0.014027997, -0.09115054, 0.13185875, -0.33627453, -0.14537394, 0.39565855, -0.475017, 0.11158323, -0.5084215, -0.31595024, -0.10161805, -0.05481188, 0.49486816, 0.6672031, 0.4663771, 0.125704, 0.24912252, 0.055553604, 0.4037343, 0.26695758, -1.1292896, 0.63466823, 0.0733432, -0.19845796, 0.07396436, 0.26503795, -0.29708937, -0.22602925, -0.2168243, 0.077604644, -0.006979987, 0.4709649, 0.079961166, -0.17668492, 0.22025388, -0.37678277, 0.030727763, -0.24768701, -0.4253156, 0.34061867, 0.23042887, -0.20976192, -0.33519247, -0.2705239, -0.30771694, 0.16828209, -0.4107083, -0.087847024, -0.017185386, -0.35532483, -0.27097744, 0.0014148615, -0.50527906, -0.030806117, 0.11612785, 0.06367651, -0.45003176, 3.8233738, -0.60224557, 0.028587867, 0.8578132, -0.75351584, 0.36815283, 0.1250936, 0.025246106, -0.18010406, -0.03302134, 0.18803245, 0.19620858, -0.3122357, 0.51300836, -0.07092627, 0.08790304, -0.08887671, 0.4246891, 0.07075959, 1.1001122, -0.21333645, 0.29649046, 0.043534737, 0.029270366, 0.026073128, 0.61024266, -0.6114707, 0.42559287, 0.61632276, -0.09138475, 0.8007551, 0.18265241, 0.8304403, -0.43585187, 0.116162986, 0.16802259, -0.1043853, -0.2539675, -0.10383299, 0.14744182, -0.19547778, -0.23289248, -0.6450098, -0.22768992, -0.9620142, -0.3408093, 0.11499898, -0.5840901, 0.1947504, -0.09942416, 0.36278287, -0.48931766, 0.2788677, -0.054631308, -0.34663615, 0.1828023, 0.082188934, 0.1821804, 0.56279206, 0.101317525, -0.023334146, -0.022398634, -0.31682587, -0.5741984, -0.12947203, -0.10002932, -1.5118301, 0.14862196, 0.16339502, -0.24742782, -0.33772773, -0.14937285, -0.043884426, -0.7241832, 0.5008706, 0.94453514, 0.4326734, -0.03361822, 0.21484517, -0.27034867, 0.28959462, 0.3566314, -0.21064529, -0.1514317, 0.12286056, -0.09480339, -0.578513, -0.11541638, 0.07288249, 0.46524373, 0.7829519, -0.31644115, -0.18785313, -0.105128735, -0.37755263, -0.42750973, -0.11714578, 0.42371383, -0.35442218, 0.1870651, -0.23002401, 1.2311393, -0.13847488, 0.07290785, 0.37778148, 0.07422729, -0.023678623, 0.55784285, 0.079697214, -0.06264907, 0.119008385, -0.3773059, -0.16150098, 1.0179288, 0.30126014, 0.31874833, -0.28532422, -0.36332735, 0.52817, -0.029264763, 0.19377504, -0.09238237, -0.038433045, -0.43449926, -2.6352246, 0.16779225, -0.043258127, 0.078494385, -0.07127215, 0.015907995, 0.51151204, -0.16836277, 0.912222, -0.05562222, 0.6734083, -0.5777303, 0.15275142, -0.42358088, -0.06890592, -0.110274255, -0.070712075, 0.055048607, -0.015897585, -0.3808212, -0.07321524, 0.21933924, 0.5140684, -0.39182618, 0.068973735, -0.4702382, 1.1164296, 1.3408012, 0.29139525, 0.356912, 0.13028303, 0.062379736, -0.3256295, -1.0198263, -0.09453055, 0.5589827, -0.27068686, -0.57935053, -0.7907647, 0.13725509, -0.025180738, 0.5580845, -0.114732, -0.14674823, -0.2035432, -0.5867202, -0.61334777, -0.05924423, -0.47192034, 0.041883558, 0.22264743, 0.07224816, 0.2563566, 0.7485367, -0.014656443, 0.033502117, -0.13618132, -0.058501884, 0.0724411, 0.24344915, -0.18524753, -0.100445405, -0.24595141, -0.03985478, 0.026785448, 0.36365286, -0.6785815, -0.29708475, 0.6678963, -0.056817513, -0.3760031, -0.13951087, 0.16512352, -0.12134847, 0.21327764, 0.07351879, -0.15478061, -0.027714524, -0.31709743, -0.24678375, 0.11009449, -0.23401015, 0.35275012, 0.09276667, 0.20977099, 0.6522212, 0.40206987, 0.036209524, 0.22082114, -0.22348009, -0.24856661, 0.12301249, 0.01069922, -0.6198362, 0.19343224, -0.31709406, -2.7746265, 0.3780414, 0.21959582, 0.21438707, -0.15561122, 0.21336496, 0.22141808, -0.08904207, -0.3144924, 0.24098182, -0.7231102, 0.0894127, -0.17156659, -0.38294476, 0.04149002, 0.081605524, -0.023402333, -0.22525011, -0.07482727, -0.40183416, -0.09354058, 0.24847814, 2.005908, -0.66135895, 0.5418649, 0.45768443, 0.09628227, 0.19701535, 0.23429017, -0.031912435, -0.42463636, -0.27534166, 0.11055295, -0.22707562, 0.1671386, 0.005982846, -0.4285896, -0.05396025, -0.05702454, -0.5207919, -0.4502285, 0.29060194, -0.23498061, 0.23953396, -0.081186146, -0.44595325, 0.038714446, -0.17040698, 0.35466185, 0.31740418, -0.21304709, -0.47570238, 0.10501945, 0.040172473, -0.11688045, -0.13195129, -0.044939656, 0.13895519, 0.40767133, -0.25359848, 0.15979119, -0.31888312, 0.77459556, 0.23360233, 0.04740978];
#[rustfmt::skip]
static LOREM_CLIP_EMB: &'static [f32] = &[-0.023821324, 0.32507098, -0.08114232, -0.01064802, -0.09680188, 0.033751644, 0.14382085, 1.0212984, 0.28260008, -0.17461702, 0.1396903, 0.1004711, 0.073565155, -0.5680146, 0.14029296, 0.07898494, 0.37258434, 0.20391336, 0.0744514, 0.040763363, 0.13386168, -0.075854294, -0.26108637, 0.107392095, -0.30911583, 0.39057928, 0.014269187, -0.07532054, 0.10831149, 0.08482613, -0.24097818, -0.014147133, 0.12065938, 0.13322037, 0.15529199, 0.007170692, -0.0058080964, -0.11849174, -0.020875335, 0.21939114, -0.12769847, -0.114053056, -0.37584096, -0.22965752, 0.14921454, -0.2620066, 0.15160947, 0.09743152, 0.03342074, -0.018973902, 0.14073929, -0.12423658, 0.07948579, 0.14987665, -0.04844691, 0.043047316, 0.2906861, -0.1467679, -0.1610454, 0.35709018, 0.02272705, -0.14923158, -0.13178478, -0.0064402223, 0.04681269, 0.115478024, -0.028862268, -0.068687506, 0.10930468, 0.30219328, 0.061279975, -0.3711514, -0.251132, 0.21756002, -0.3937102, 0.061121836, 0.13895264, -0.051983945, 0.042168755, 0.15156946, -0.24410763, 0.18481277, 0.06010418, 0.0047012568, -0.2167329, 0.17586204, 0.21929322, -0.3607635, 0.16905123, -0.38552964, 0.103187956, 0.039027445, -1.7404354, 0.12496659, -0.20562452, -0.44134653, -0.062468424, 0.124385364, 0.09474781, -0.38050294, 0.015883297, 0.020162033, 0.2917977, 0.2804438, 0.31336206, 0.016228497, -0.37608927, 0.21808012, -0.07862981, -0.14851817, -0.10471277, 0.3876447, -0.01984132, -0.07072827, -0.22728856, -0.18501802, -0.060585007, -0.21040738, -0.015098579, -0.16300584, -0.02612365, 0.01272653, -0.28801012, -0.31140608, -0.3180123, 0.12075325, 0.04442505, 0.18702894, -0.5647113, 0.20522851, 0.06007894, -0.13508567, -0.05381289, 7.3227267, 0.23174843, 0.26095513, -0.12765212, 0.100984484, -0.09794246, 0.067181766, 0.17050412, -0.23870692, 0.016411357, -0.054422162, -0.1647437, 0.2181981, 0.11651408, -0.008177299, -0.22514509, -0.18892638, 0.3044839, -0.31126016, -0.113348305, -0.24158034, -0.4983992, -0.376536, 0.14073405, -0.0849548, 0.08642887, -0.034770034, 0.066097856, 0.17894936, -0.19096602, 0.10812106, 0.04696764, 0.11783227, 0.2508147, -0.20124456, 0.044359636, 0.13551393, 0.100525826, 0.17257476, 0.06682797, -0.40206867, 0.32027435, -0.22901152, -0.078228116, 0.007135289, 0.23757754, -0.14507365, 0.020956187, 0.031298384, 0.043612532, 0.20656097, -0.34732053, 0.094974935, 0.22666313, 0.02207933, -0.19816363, -0.30798194, -0.30086106, -0.059007052, 0.03745848, -0.33173567, -0.06358832, 0.24264155, 0.019742087, -0.14840543, 0.08243634, -0.029046342, 0.011549339, -0.19661047, -0.28590125, -0.07922155, -0.17834195, -0.18280725, 0.20806038, 0.15688741, -0.2910517, 0.033820882, 0.23478493, -0.085333735, -0.06174528, -0.30954897, -0.22866535, -0.0066644624, 0.13974859, 0.045060627, 0.4538477, -0.08859053, 0.353545, -0.10418525, 0.15865125, -0.10507354, -0.038068682, 0.24129772, 0.33472478, 0.06640434, -0.17550236, -0.239943, 0.058750305, -0.10108164, -0.08837568, -0.0829607, 0.32146657, -0.29684764, 0.38338006, 0.15715083, -0.17116335, 0.0990006, 0.06986122, 0.66042197, 0.7861764, -0.17475459, -0.132988, 0.37950087, -0.21996504, 0.062849775, 0.28789008, 0.18028432, -0.04843106, 0.19591928, -0.12724626, -0.023116939, 0.09516066, -0.24818723, 0.28573015, 0.15771933, 0.34037498, 0.08924934, -0.15265608, -0.026823826, 0.28749287, 0.09055933, 0.114482395, 0.14233656, -0.30804974, 0.020725299, -0.10022853, -0.010006551, -0.23381731, 0.33186394, 0.292484, -0.20212, -0.18317077, 0.03704127, 0.17566401, -0.07680078, -0.33329242, -0.1509992, 0.05023521, -0.16249955, -0.15484932, 0.017528035, 0.16193792, -0.63092536, -0.05514544, -0.26290503, 0.20170087, 0.27672285, -0.18762435, 0.15048854, -0.33022165, 0.021839358, 0.13971621, -0.07167573, -0.31037253, 0.10240001, -0.028671501, 0.11374885, 0.23593777, 0.029892594, 0.1946212, 0.19532663, 0.20063168, 0.5550091, -0.28335252, -0.03726676, -0.11575253, 0.22081777, -0.0003119558, 0.15380046, 7.3187094, 0.2863891, 0.16957754, 0.20766485, -0.18246536, -0.06703475, -0.017270312, -0.35771132, -0.19213994, 0.5207134, -0.10775176, -0.07050047, -0.19999668, -0.090156525, 0.31709296, 0.002506569, 0.16990559, -1.8464317, 0.4529942, -0.16174158, -0.22049534, 0.0011903644, -0.33204472, 0.023596093, 0.14144875, 0.13108099, -0.27007574, -0.16729395, 0.2593482, -0.3453375, -0.23076789, -0.03902781, -0.010879591, -0.07499196, 0.02078572, -0.01114741, -0.0034915805, 0.5825228, -0.6007199, -0.07453986, -0.22106047, 0.31835693, 0.05894721, 0.114553496, 0.022928156, -0.37448475, 0.16037507, 0.1191238, -0.019062236, 0.0985478, -0.050559677, -0.12502712, -0.1540978, 0.02884806, -0.08804091, 0.122339025, 0.16702324, 0.03476978, 0.10906562, -0.09596213, 0.046655715, 0.27750963, -0.13844311, 0.23404725, 0.5224707, -0.13118605, 0.053349733, -0.15122356, -0.49718392, -0.20539615, -0.21734503, 0.0027721412, 0.4145457, 0.11424269, 0.18566465, 0.15300472, 0.30530688, 0.35333866, 0.044030994, -0.21172473, 0.3279033, 0.2540502, -0.19065252, 0.33100513, -0.0441064, 0.38362837, 0.044554256, 0.6020556, 0.11967351, 0.12491087, -0.19261467, -0.34304425, -0.22187197, 0.12404098, -0.1469062, -0.014435895, 0.014551206, 0.19680694, -0.12833068, -0.33180538, 0.21671897, 0.00010409951, 0.11341271, -0.0568607, 0.23516011, -0.3495756, -0.43202823, -0.356337, 0.1195309, -0.31215852, 0.10391501, -0.23814733, -0.18927348, -0.13337821, 0.430021, -0.19487298, -0.6941422, -0.00026670098, 0.12753001, -0.21944714, -0.0660056, -0.4212317, -0.15676711, -0.04230821, 0.050726086, -0.07771948, 0.24228217, -0.38280416, -0.04808271, 0.29916406, -0.30997515, -0.12363116, 0.120864496, 0.061819322, -0.00026150793, -0.2634592, 0.09269245, -0.32996154, 0.1882259, 0.16159256, -0.10508378, 0.37686154, 0.30355656, 0.19205064, 0.3257804, -0.05408972, -0.2193405, -0.05558881, -0.086777374, -0.2449651, 0.2983935, -0.2777422, -0.15365465, -0.25493696, -0.146432, -0.25489265, -0.29967153, -0.2748564, 0.274489, 0.45283753, -0.13507074, 0.15974323, 0.15124017, 0.1591709, -0.2105168, -0.068934366, 0.03678081, 0.026063636, -0.123701155, -1.8959751, -0.20136517, -0.26924673, -0.07676838, 0.20661148, 0.063208096, 0.025541574, 0.2215255, 0.04044193, 0.28281662, -0.027299553, -0.26696467, -0.30052483, -0.19038662, -0.022349298, -0.20905101, -0.096490055, -0.53103006, -0.0001690872, -0.13068888, -0.20970923, 0.13387391, -0.23632008, 0.014863417, 0.15023494, -0.27275097, 0.1622324, 0.056410834, -1.2211498, -0.04189681, -0.09479007];
#[rustfmt::skip]
static LOREM_BGE_SMALL_EMB: &'static [f32] = &[-0.030880496, -0.11021069, 0.3917847, -0.35962796, 0.22797719, 0.1286265, 0.104809806, 0.4536326, 0.20183088, 0.1336294, -0.028951608, -0.3623364, 0.088983126, 0.46398085, 0.3589503, 0.21120659, 0.33224252, -0.12301457, -1.0269446, -0.19482881, 1.0011039, 0.4460324, -0.19272235, -0.47009823, 0.009932369, -0.14311227, -0.13751373, 0.3344442, -0.014674753, -1.6587354, -0.13096964, -0.11285492, 0.91281325, -0.099221066, 0.35517678, -0.17221186, -0.023663897, 0.45018157, -0.42092425, 0.07768755, 0.39460424, -0.11940934, -0.014027997, -0.09115054, 0.13185875, -0.33627453, -0.14537394, 0.39565855, -0.475017, 0.11158323, -0.5084215, -0.31595024, -0.10161805, -0.05481188, 0.49486816, 0.6672031, 0.4663771, 0.125704, 0.24912252, 0.055553604, 0.4037343, 0.26695758, -1.1292896, 0.63466823, 0.0733432, -0.19845796, 0.07396436, 0.26503795, -0.29708937, -0.22602925, -0.2168243, 0.077604644, -0.006979987, 0.4709649, 0.079961166, -0.17668492, 0.22025388, -0.37678277, 0.030727763, -0.24768701, -0.4253156, 0.34061867, 0.23042887, -0.20976192, -0.33519247, -0.2705239, -0.30771694, 0.16828209, -0.4107083, -0.087847024, -0.017185386, -0.35532483, -0.27097744, 0.0014148615, -0.50527906, -0.030806117, 0.11612785, 0.06367651, -0.45003176, 3.8233738, -0.60224557, 0.028587867, 0.8578132, -0.75351584, 0.36815283, 0.1250936, 0.025246106, -0.18010406, -0.03302134, 0.18803245, 0.19620858, -0.3122357, 0.51300836, -0.07092627, 0.08790304, -0.08887671, 0.4246891, 0.07075959, 1.1001122, -0.21333645, 0.29649046, 0.043534737, 0.029270366, 0.026073128, 0.61024266, -0.6114707, 0.42559287, 0.61632276, -0.09138475, 0.8007551, 0.18265241, 0.8304403, -0.43585187, 0.116162986, 0.16802259, -0.1043853, -0.2539675, -0.10383299, 0.14744182, -0.19547778, -0.23289248, -0.6450098, -0.22768992, -0.9620142, -0.3408093, 0.11499898, -0.5840901, 0.1947504, -0.09942416, 0.36278287, -0.48931766, 0.2788677, -0.054631308, -0.34663615, 0.1828023, 0.082188934, 0.1821804, 0.56279206, 0.101317525, -0.023334146, -0.022398634, -0.31682587, -0.5741984, -0.12947203, -0.10002932, -1.5118301, 0.14862196, 0.16339502, -0.24742782, -0.33772773, -0.14937285, -0.043884426, -0.7241832, 0.5008706, 0.94453514, 0.4326734, -0.03361822, 0.21484517, -0.27034867, 0.28959462, 0.3566314, -0.21064529, -0.1514317, 0.12286056, -0.09480339, -0.578513, -0.11541638, 0.07288249, 0.46524373, 0.7829519, -0.31644115, -0.18785313, -0.105128735, -0.37755263, -0.42750973, -0.11714578, 0.42371383, -0.35442218, 0.1870651, -0.23002401, 1.2311393, -0.13847488, 0.07290785, 0.37778148, 0.07422729, -0.023678623, 0.55784285, 0.079697214, -0.06264907, 0.119008385, -0.3773059, -0.16150098, 1.0179288, 0.30126014, 0.31874833, -0.28532422, -0.36332735, 0.52817, -0.029264763, 0.19377504, -0.09238237, -0.038433045, -0.43449926, -2.6352246, 0.16779225, -0.043258127, 0.078494385, -0.07127215, 0.015907995, 0.51151204, -0.16836277, 0.912222, -0.05562222, 0.6734083, -0.5777303, 0.15275142, -0.42358088, -0.06890592, -0.110274255, -0.070712075, 0.055048607, -0.015897585, -0.3808212, -0.07321524, 0.21933924, 0.5140684, -0.39182618, 0.068973735, -0.4702382, 1.1164296, 1.3408012, 0.29139525, 0.356912, 0.13028303, 0.062379736, -0.3256295, -1.0198263, -0.09453055, 0.5589827, -0.27068686, -0.57935053, -0.7907647, 0.13725509, -0.025180738, 0.5580845, -0.114732, -0.14674823, -0.2035432, -0.5867202, -0.61334777, -0.05924423, -0.47192034, 0.041883558, 0.22264743, 0.07224816, 0.2563566, 0.7485367, -0.014656443, 0.033502117, -0.13618132, -0.058501884, 0.0724411, 0.24344915, -0.18524753, -0.100445405, -0.24595141, -0.03985478, 0.026785448, 0.36365286, -0.6785815, -0.29708475, 0.6678963, -0.056817513, -0.3760031, -0.13951087, 0.16512352, -0.12134847, 0.21327764, 0.07351879, -0.15478061, -0.027714524, -0.31709743, -0.24678375, 0.11009449, -0.23401015, 0.35275012, 0.09276667, 0.20977099, 0.6522212, 0.40206987, 0.036209524, 0.22082114, -0.22348009, -0.24856661, 0.12301249, 0.01069922, -0.6198362, 0.19343224, -0.31709406, -2.7746265, 0.3780414, 0.21959582, 0.21438707, -0.15561122, 0.21336496, 0.22141808, -0.08904207, -0.3144924, 0.24098182, -0.7231102, 0.0894127, -0.17156659, -0.38294476, 0.04149002, 0.081605524, -0.023402333, -0.22525011, -0.07482727, -0.40183416, -0.09354058, 0.24847814, 2.005908, -0.66135895, 0.5418649, 0.45768443, 0.09628227, 0.19701535, 0.23429017, -0.031912435, -0.42463636, -0.27534166, 0.11055295, -0.22707562, 0.1671386, 0.005982846, -0.4285896, -0.05396025, -0.05702454, -0.5207919, -0.4502285, 0.29060194, -0.23498061, 0.23953396, -0.081186146, -0.44595325, 0.038714446, -0.17040698, 0.35466185, 0.31740418, -0.21304709, -0.47570238, 0.10501945, 0.040172473, -0.11688045, -0.13195129, -0.044939656, 0.13895519, 0.40767133, -0.25359848, 0.15979119, -0.31888312, 0.77459556, 0.23360233, 0.04740978];
#[rustfmt::skip]
static LOREM_TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi eget nisi eget enim scelerisque bibendum. Praesent efficitur risus ut nunc placerat vehicula. Integer ut leo luctus, aliquam sem id, suscipit urna. Aenean convallis tellus at tortor efficitur pulvinar. Nunc et lobortis dolor. Sed faucibus, sapien eu commodo lobortis, enim sapien sagittis nisl, quis molestie sapien diam posuere mi. Donec lacus ex, scelerisque a bibendum at, ornare in massa. Pellentesque vel sollicitudin purus. Nunc varius erat a lacus tempus pulvinar. Sed et vulputate velit. Mauris a leo hendrerit ipsum tempus fringilla. Sed venenatis est quis dolor egestas, eu condimentum ipsum laoreet. Sed imperdiet vitae lacus sed dictum. Nunc eget neque a odio commodo fermentum at semper massa. Aenean ut sem ac felis hendrerit venenatis nec in arcu. Ut blandit quam eu turpis rutrum, vitae blandit purus luctus. Integer molestie varius neque quis bibendum. Phasellus efficitur at dolor cursus tincidunt. Cras quam sem, lobortis eu erat et, tristique accumsan orci. Ut eu ipsum quis orci rhoncus pulvinar. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae;";

macro_rules! text_embedding_test {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let runtime = get_runtime(&Runtime::Ort, None, "{\"data_path\": \"/tmp/lantern-embeddings-core-test\"}").unwrap();
            let (model, input, expected, batch_size) = $value;
            let inputs = itertools::repeat_n(input, batch_size).collect();
            let output = runtime.process(
                model,
                &inputs,
            ).unwrap();

            let expected_output: Vec<Vec<f32>> =
                itertools::repeat_n(expected, batch_size).collect();
            assert_eq!(output, expected_output);
        }
    )*
    }
}

text_embedding_test! {
  generate_clip_embeddings_small_text_single: ("clip/ViT-B-32-textual", HELLO_WORLD_TEXT, HELLO_WORLD_CLIP_EMB.to_vec(), 1),
  generate_clip_embeddings_small_text_batch: ("clip/ViT-B-32-textual", HELLO_WORLD_TEXT, HELLO_WORLD_CLIP_EMB.to_vec(), 10),
  generate_clip_embeddings_large_text_single: ("clip/ViT-B-32-textual", LOREM_TEXT, LOREM_CLIP_EMB.to_vec(), 1),
  generate_clip_embeddings_large_text_batch: ("clip/ViT-B-32-textual", LOREM_TEXT, LOREM_CLIP_EMB.to_vec(), 50),
  generate_bge_embeddings_small_text_single: ("BAAI/bge-small-en", HELLO_WORLD_TEXT, HELLO_WORLD_BGE_SMALL_EMB.to_vec(), 1),
  generate_bge_embeddings_small_text_batch: ("BAAI/bge-small-en", HELLO_WORLD_TEXT, HELLO_WORLD_BGE_SMALL_EMB.to_vec(), 100),
  generate_bge_embeddings_large_text_single: ("BAAI/bge-small-en", HELLO_WORLD_TEXT, LOREM_BGE_SMALL_EMB.to_vec(), 1),
  generate_bge_embeddings_large_text_batch: ("BAAI/bge-small-en", HELLO_WORLD_TEXT, LOREM_BGE_SMALL_EMB.to_vec(), 100),
}
