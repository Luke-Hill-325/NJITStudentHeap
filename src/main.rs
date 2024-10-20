use rand::prelude::*;
use rand::distributions::WeightedIndex;

const NUMSTUDENTS: u32 = 12000;
const N: usize = 15000;

struct MyHashingHeap<T: Clone> {
    primary_storage: Vec<Option<(u32, T)>>,
    collision_count: u32
}
fn main() {
    let ucid_list: Vec<String> = gen_ucids();
    //let ucid_nums: Vec<u32> = ucid_list.iter().map(|ucid| ucid_to_int(ucid)).collect();
    assert_eq!(12000, ucid_list.iter().map(|n| ucid_list.iter().filter(|x| n == *x).count()).filter(|&j| j == 1).count());
    assert_eq!(12000, ucid_list.len());

    let mut rng = thread_rng();

    let mut ucid_heap: MyHashingHeap<String> = MyHashingHeap::instantiate();
    for ucid in ucid_list.into_iter() {
        ucid_heap.put(&ucid, ucid.clone());
    }
    println!("Collisions: {}", ucid_heap.collision_count);
}

fn ucid_to_int(ucid: &str) -> u32 {
    let (initials, numeric) = ucid.split_at(2);
    let mut alphabetics = initials.chars().map(|c| c.to_digit(36).unwrap() - 10);
    return (alphabetics.next().unwrap() * 26 + alphabetics.next().unwrap()) * 26 * 10 * 10 * 10 + numeric.parse::<u32>().expect("bad num");
}
    
impl<T: Clone> MyHashingHeap<T> {
    pub fn instantiate() -> Self {
       Self {
           primary_storage: vec![None; N],
           collision_count: 0
       } 
    }

    fn hash(&mut self, key: u32) -> u32 {
        let hash_key = key % N as u32;
        if let Some(kv) = self.primary_storage[hash_key as usize].clone() {
            if kv.0 != hash_key {
                self.collision_count += 1;
            }
        }
        hash_key
    }

    pub fn get(&mut self, key: &str) -> Option<T> {
        let key_as_num = ucid_to_int(key);
        let k = self.hash(key_as_num);
        let mut i = k;
        while let Some(kv) = self.primary_storage[i as usize].clone() {
            if kv.0 == key_as_num {
                return Some(kv.1)
            }
            i = (i + 1) % N as u32;
            assert_ne!(i, k);
        }
        None
    }

    pub fn put(&mut self, key: &str, val: T) -> Option<T> {
        let key_as_num = ucid_to_int(key);
        let k = self.hash(key_as_num);
        let mut i = k;
        while let Some(kv) = self.primary_storage[i as usize].clone() {
            if kv.0 == key_as_num {
                self.primary_storage[i as usize] = Some((key_as_num, val));
                return Some(kv.1)
            }
            i = (i + 1) % N as u32;
            assert_ne!(i, k);
        }
        self.primary_storage[i as usize] = Some((key_as_num, val));
        None
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        let key_as_num = ucid_to_int(key);
        let k = self.hash(key_as_num);
        let mut i = k;
        while let Some(kv) = self.primary_storage[i as usize].clone() {
            if kv.0 == key_as_num {
                self.primary_storage[i as usize] = None;
                self.shift_begin(i);
                return Some(kv.1);
            }
            i = (i + 1) % N as u32;
            assert_ne!(i, k);
        }
        None
    }


    fn shift(&mut self, i: u32, mut min_valid: i32) {
        while let Some(kv) = self.primary_storage[((min_valid - 1 + N as i32) % N as i32) as usize].clone() {
            min_valid -= 1;
        }
        let mut last_available = i;
        let mut s = 1;
        while let Some(kv) = self.primary_storage[((i + s) % N as u32) as usize].clone() {
            if kv.0 < i {
                if min_valid < 0 || kv.0 as i32 >= min_valid {
                    last_available = (i + s) % N as u32;
                }
            } else if min_valid < 0 && kv.0 as i32 >= N as i32 + min_valid {
                last_available = (i + s) % N as u32;
            }
            s += 1;
        }
        if last_available != i {
            self.primary_storage[i as usize] = self.primary_storage[last_available as usize].clone(); 
            self.primary_storage[last_available as usize] = None;
            self.shift(last_available, min_valid);
        }
    }
    fn shift_begin(&mut self, i: u32) {
        self.shift(i, i as i32);
    }
}

fn gen_ucids() -> Vec<String> {
    //https://www.ssa.gov/cgi-bin/namesbystate.cgi
    let firstnames2003 = "meminojeasrscadjakjmjjjnaadvjgjbkstabgwa\
                          tljechrrjazadabkjnsaakejkgbmgajmsalssspr\
                          ctemesjmadejipbhjecjnbakaacglcrdscactavj\
                          lmgsjfiklmjapmlmoljmncjkmksvxmamjjngbhcl\
                          asejszjdjiaadmhrpacanmjcdsjajbkjcjfemaga".chars();
    let firstnames2003weights: [u32; 200] = [1340, 818, 1226, 564, 1030, 563, 925, 540, 887, 508, 878, 502, 858, 498, 852, 471, 758, 470, 695, 439, 664, 434, 656, 403, 588, 394, 583, 392, 556, 379, 514, 375, 512, 361, 507, 359, 499, 352, 497, 324, 496, 324, 461, 322, 452, 314, 443, 312, 433, 277, 431, 275, 428, 275, 423, 270, 415, 253, 415, 251, 399, 250, 349, 249, 336, 248, 332, 238, 297, 237, 280, 236, 280, 235, 264, 231, 256, 206, 254, 203, 246, 201, 240, 196, 240, 186, 237, 183, 235, 181, 234, 181, 214, 180, 210, 171, 208, 170, 204, 169, 202, 167, 199, 162, 195, 158, 194, 157, 193, 156, 192, 156, 190, 154, 189, 152, 189, 151, 189, 151, 178, 150, 173, 150, 169, 147, 164, 145, 164, 145, 162, 137, 161, 137, 159, 135, 159, 134, 156, 133, 155, 123, 153, 119, 153, 119, 150, 118, 149, 117, 148, 112, 148, 111, 143, 109, 142, 109, 141, 109, 134, 107, 134, 107, 133, 106, 127, 105, 126, 104, 126, 104, 126, 104, 126, 104, 126, 103, 120, 103, 111, 103, 110, 101, 109, 100, 108, 99, 107, 99, 106, 96, 104, 96, 103, 95, 103, 92, 102, 90];
    
    let firstnames2004 = "memonirsjaaedkcmjsajjsaadgknjjjajgtbwvca\
                          betadajleranjabmjkrhzgsakjlacabsgkssjsem\
                          eschpjatbrcagllmemneamlesmlbrgnpacjkakjj\
                          vdidjaofsmlmxaaciajdjccmnmsrdctjjljvejpg\
                          ascjmzanpjjajmajgbjmbanaakdaocjkmmkacdga".chars();
    let firstnames2004weights: [u32; 200] = [1217, 780, 1092, 569, 987, 564, 939, 555, 931, 539, 810, 508, 805, 482, 799, 480, 678, 461, 658, 427, 642, 420, 607, 376, 579, 371,539, 366, 536, 363, 527, 357, 501, 351, 501, 346, 498, 331, 496, 314, 474, 296, 467, 285, 424, 285, 420, 278, 409, 270, 403, 256, 388, 248, 382, 246, 380, 243, 370, 240, 366, 238, 356, 236, 318, 222, 313, 217, 311, 215, 305, 212, 283, 201, 283, 198, 265, 198, 255, 185, 242, 185, 236, 184, 236, 184, 233, 180, 228, 178, 219, 177, 216, 176, 216, 175, 214, 174, 210, 171, 209, 165, 207, 160, 201, 160, 200, 156, 199, 154, 198, 154, 197, 151, 193, 149, 188, 144, 187, 143, 184, 142, 180, 140, 179, 138, 176, 138, 166, 138, 165, 137, 163, 134, 156, 133, 155, 132, 155, 132, 155, 131, 153, 128, 153, 127, 150, 125, 142, 123, 141, 119, 137, 119, 137, 116, 134, 112, 134, 111, 133, 108, 132, 106, 131, 106, 125, 105, 123, 104, 119, 103, 118, 103, 114, 102, 113, 101, 111, 100, 107, 99, 107, 99, 105, 99, 105, 97, 104, 95, 103, 95, 103, 93, 99, 89, 98, 89, 98, 88];

    let firstnames2005 = "memiraaojsnmcadsjeajjsaadkjmtjjgwgbnjbke\
                          cajvjatndaeragrhjazlbkbsgksaeacajjselsjh\
                          kmnbjsealjatnlcggsembmarpjlcaradleslsmim\
                          cmopjcafvkraiglmjjxctvjknmckmcmmpdjjeajc\
                          daajjiamgcpmjnjesjckssczadiknsjemkggjmva".chars();
    let firstnames2005weights: [u32; 200] = [1085, 702, 972, 559, 920, 548, 866, 503, 852, 493, 817, 491, 782, 476, 760, 471, 690, 463, 638, 429, 630, 410, 609, 374, 557, 362, 521, 344, 519, 343, 489, 342, 484, 340, 477, 300, 473, 298, 458, 283, 445, 280, 440, 278, 432, 277, 427, 277, 424, 272, 408, 272, 374, 266, 361, 251, 342, 248, 328, 235, 325, 230, 324, 227, 321, 224, 319, 218, 305, 206, 283, 206, 279, 193, 275, 183, 267, 179, 258, 176, 255, 170, 248, 168, 238, 168, 236, 166, 232, 164, 222, 164, 218, 160, 211, 158, 211, 158, 209, 153, 208, 151, 207, 148, 207, 147, 203, 146, 199, 145, 195, 143, 193, 143, 192, 143, 190, 142, 187, 138, 183, 137, 182, 137, 181, 132, 174, 132, 164, 130, 162, 127, 161, 123, 160, 122, 156, 120, 154, 119, 149, 119, 147, 118, 146, 118, 145, 117, 143, 111, 140, 110, 139, 108, 137, 108, 133, 106, 133, 104, 132, 102, 131, 100, 131, 99, 127, 99, 126, 96, 124, 96, 121, 94, 119, 91, 119, 91, 115, 91, 115, 91, 114, 91, 112, 89, 112, 89, 111, 89, 109, 88, 108, 88, 105, 87, 105, 87, 104, 86];
    let firstnames2006 = "mimadejsaorsnacmjmasjjaedajktgjgjbjabakv\
                          cjwajgdhantaseesjnbkrajlgezslkjablshnrac\
                          lmebajkmealagmcsbjnrjmsrigekaljmvaamcmjm\
                          oasjatlgnsxpcedfpjlkcdrdpkivjjmaccanjjjc\
                          ajcjjsgctiajhkcmnadmjmmmjasaizjaeaedkcme".chars();
    let firstnames2006weights: [u32; 200] = [1029, 652, 901, 640, 794, 618, 789, 494, 783, 481, 752, 466, 749, 450, 723, 440, 649, 410, 607, 410, 569, 409, 554, 408, 502, 382, 477, 377, 466, 359, 464, 335, 453, 327, 452, 285,444, 285, 429, 283, 422, 264, 422, 260, 420, 260, 415, 255, 406, 251, 394, 241, 361, 236, 351, 236, 346, 234, 344, 233, 335, 220, 329, 219, 304, 217, 301, 213, 300, 204, 294, 197, 291, 196, 286, 195, 277, 187, 268, 181, 260, 178, 258, 167, 257, 167, 247, 166, 242, 162, 239, 161, 234, 158, 232, 158, 227, 156, 222, 156, 216, 154, 216, 150, 211, 143, 203, 143, 201, 143, 200, 141, 194, 139, 193, 139, 190, 138, 190, 136, 190, 134, 187, 134, 178, 134, 178, 133, 178, 130, 176, 125, 175, 117, 174, 118, 174, 114, 171, 114, 167, 113, 162, 112, 148, 112, 145, 112, 144, 111, 144, 109, 143, 109, 141, 109, 139, 108, 137, 105, 136, 105, 135, 105, 134, 103, 130, 102, 124, 99, 123, 96, 122, 96, 119, 96, 119, 95, 118, 95, 113, 95, 113, 95, 112, 93, 112, 92, 111, 92, 108, 91, 105, 91, 105, 91, 103, 89, 100, 88];

    //https://unmask.com/most-popular-last-names/state/NJ/
    let lastnames: Vec<u32> = "svplbcbklnmsrlmcgkskphdsmltdhcldwhmmcjchtbbdsmrnszlhmbhcdrmfswmpsfmmwhmpcscksssa\
                     omjaflpnhfmdkghhhpcimdhzcmgpmmfmcwmrjgqrpirmjwmpnwrdcnearraclitfmhlnsgfwpmpscmfb\
                     lnsjarhccmhgvbeyscwmjaesabkwgcldtbbhrgngcbdhcbmtcndhbbsvynsbrmggmbmhnaskvfkcmysf\
                     rvecptlgpgwkhjnlbascghaohesmfpbvbgtdgabcjkcmcwzdmppbpmdiwdtcdewohkmvgcplmffhrskm\
                     dmmaphcpccmdhacarjodpcmthmccsbcfvbddcocapmwiardbgbchhamtvpprhfmpscncllpcccdmlbsk\
                     dmcmbmcraybbsmrblalpbhcbhcsmembowgwzcschbrlmrtvshfkcgsgfrevgtmgcltfkmmsrfgqbbfmm\
                     ofkmpmasrdkrmdbwsswmcbhdnmlhbdbmspmapcmlmpccackjmtccwcfcjcknghbrdsswclvasiimcdbg\
                     wcslashgohpmbwmdmlfmmmlggnnesbthbbsdhdabgchamgmembmrtscvbccrnarlgmdwdabfaffeokhc\
                     clddwwfsplglmdwfcjsgektmakrabamgnrmssnbybsfwwepidkhkkebrloescclghfbhkymsmlqgmffl\
                     dhsdshhmfphcblbpmrfbsvdlrsvdmgjwcwlcmbkdcqytcmrhafpgdmvrvhcgfmccamfrrscndkgeamss\
                     lgrebnpbfpwabdwhrpoldefcvtpswrlvmcmwaspaphpbavbbpddahshgpsdcgdbdhtlnygrtbtbdaero\
                     smcmirrmxplrmjrrhrhgbrcfkmshramlwhmlmcbbscsfmwomuhmmbmhrmmpgmkvebdmadtamkschkbsd\
                     fmtmgblwabbafalddbbhdgcowmdrtbbtpdcddwep".chars().map(|c| c.to_digit(36).unwrap() - 10).collect();
    let lastnameweights: [usize; 1000] = [10935, 1174, 770, 603, 10077, 1173, 769, 602, 9803, 1173, 769, 602, 9803, 1173, 769, 601, 9152, 1168, 767, 601, 6422, 1165, 764, 601, 6394, 1163, 764, 600, 6123, 1159, 764, 600, 5825, 1153, 763, 600, 5603, 1149, 763, 600, 5127, 1148, 762, 600, 5023, 1147, 761, 5999, 4971, 1145, 761, 599, 4902, 1144, 760, 599, 4745, 1143, 756, 598, 4651, 1143, 756, 598, 4569, 1139, 756, 597, 4030, 1137, 755, 597, 3980, 1133, 755, 597, 3942, 1132, 755, 596, 3915, 1131, 755, 596, 3909, 1129, 754, 596, 3795, 1129, 754, 596, 3664, 1129, 754, 596, 3660, 1123, 754, 595, 3628, 1122, 750, 594, 3596, 1119, 750, 594, 3548, 1117, 750, 593, 3537, 1116, 749, 592, 3499, 1116, 749, 591, 3436, 1115, 746, 590, 3370, 1110, 746, 589, 3232, 1102, 746, 589, 3206, 1100, 745, 589, 3098, 1099, 745, 589, 3034, 1094, 744, 589, 3020, 1093, 744, 588, 2988, 1092, 742, 587, 2987, 1087, 742, 587, 2917, 1079, 741, 587, 2902, 1067, 741, 586, 2882, 1065, 741, 585, 2819, 1064, 740, 584, 2814, 1063, 739, 584, 2788, 1062, 739, 583, 2788, 1061, 737, 583, 2779, 1059, 737, 5582, 2775, 1054, 735, 581, 2774, 1054, 734, 579, 2763, 1053, 733, 578, 2722, 1046, 733, 577, 2707, 1045, 732, 577, 2684, 1045, 732, 577, 2622, 1042, 732, 577, 2618, 1042, 731, 576, 2540, 1042, 731, 575, 2540, 1042, 729, 575, 2516, 1041, 729, 575, 2512, 1041, 729, 575, 2486, 1040, 727, 574, 2422, 1039, 727, 573, 2421, 1036, 726, 572, 2421, 1033, 725, 572, 2413, 1033, 725, 572, 2392, 1032, 723, 571, 2391, 1031, 722, 571, 2379, 1030, 720, 571, 2367, 1028, 719, 570, 2363, 1026, 719, 570, 2355, 1025, 718, 570, 2336, 1023, 716, 569, 2331, 1023, 716, 569, 2322, 1021, 716, 569, 2296, 1020, 715, 568, 2295, 1019, 714, 567, 2285, 1019, 714, 566, 2281, 1018, 713, 566, 2277, 1014, 712, 564, 2277, 1014, 712, 564, 2261, 1004, 712, 564, 2258, 1002, 709, 564, 2228, 1001, 708, 563, 2104, 997, 708, 563, 2102, 995, 706, 562, 2096, 991, 704, 561, 2086, 991, 703, 561, 2084, 989, 702, 561, 2082, 989, 702, 561, 2081, 988, 702, 561, 2073, 987, 700, 559, 2069, 986, 699, 559, 2064, 984, 698, 559, 2035, 971, 698, 559, 2033, 969, 698, 558, 2010, 968, 697, 558, 1998, 966, 693, 558, 1992, 964, 692, 556, 1947, 962, 692, 555, 1930, 961, 692, 555, 1922, 957, 692, 555, 1921, 1921, 1920, 1911, 1892, 1859, 1848, 1846, 1833, 1828, 1827, 1823, 1814, 1796, 1792, 1764, 1761, 1754, 1752, 1743, 955, 955, 953, 953, 949, 948, 948, 947, 943, 942, 941, 941, 941, 940, 937, 935, 933, 931, 931, 923, 690, 690, 689, 689, 688, 687, 686, 685, 684, 684, 684, 683, 683, 681, 680, 680, 680, 679, 679, 679, 554, 554, 554, 554, 553, 553, 553, 553, 553, 551, 550, 550, 549, 547, 547, 546, 546, 546, 545, 544, 1738, 1706, 1704, 1702, 1701, 1697, 1686, 1678, 1668, 1660, 1660, 1652, 1643, 1624, 1620, 1620, 1614, 1614, 1608, 1603, 923, 921, 917, 913, 911, 910, 910, 908, 907, 904, 903, 902, 902, 902, 901, 896, 894, 893, 893, 891, 678, 677, 676, 675, 674, 674, 674, 671, 669, 667, 666, 666, 666, 665, 665, 664, 664, 664, 664, 663, 544, 544, 544, 543, 543, 542, 541, 541, 541, 541, 540, 540, 539, 539, 539, 539, 538, 537, 537, 536, 1602, 1596, 1584, 1582, 1581, 1573, 1567, 1559, 1559, 1555, 1544, 1535, 1533, 1533, 1525, 1524, 1517, 1516, 1515, 1514, 889, 889, 887, 885, 884, 884, 883, 881, 880, 880, 878, 878, 877, 875, 875, 875, 875, 875, 874, 874, 662, 662, 661, 658, 658, 658, 657, 656, 656, 655, 655, 654, 653, 653, 653, 652, 652, 652, 651, 650, 536, 534, 534, 533, 533, 533, 532, 532, 531, 531, 531, 531, 531, 530, 530, 529, 528, 528, 528, 527, 1496, 1495, 1495, 1494, 1493, 1472, 1468, 1464, 1455, 1455, 1453, 1437, 1435, 1434, 1428, 1426, 1410, 1405, 1405, 1403, 873, 873, 872, 871, 870, 869, 869, 869, 868, 865, 865, 865, 864, 863, 863, 861, 859, 859, 858, 857, 648, 647, 646, 646, 646, 646, 645, 645, 643, 643, 642, 642, 641, 641, 641, 641, 639, 639, 639, 638, 527, 527, 527, 527, 526, 526, 525, 525, 524, 524, 524, 523, 523, 522, 522, 522, 522, 521, 520, 520, 1401, 1398, 1395, 1395, 1391, 1391, 1384, 1383, 1380, 1377, 1377, 1369, 1367, 1363, 1363, 1354, 1350, 1350, 1348, 1345, 856, 856, 856, 855, 855, 853, 853, 848, 848, 847, 847, 846, 845, 843, 840, 839, 836, 836, 834, 834, 638, 637, 636, 635, 635, 634, 634, 632, 631, 630, 630, 630, 630, 630, 630, 629, 628, 627, 626, 626, 520, 520, 519, 519, 518, 517, 517, 517, 517, 517, 516, 516, 516, 516, 515, 515, 514, 513, 513, 513, 1341, 1335, 1330, 1327, 1325, 1324, 1319, 1316, 1303, 1303, 1303, 1300, 1294, 1291, 1284, 1283, 1279, 1276, 1273, 1270, 834, 834, 831, 831, 829, 829, 827, 827, 824, 824, 823, 820, 814, 814, 812, 812, 812, 808, 808, 808, 626, 626, 626, 624, 623, 623, 623, 623, 623, 623, 622, 622, 621, 621, 621, 620, 620, 620, 619, 618, 513, 513, 513, 512, 512, 512, 512, 512, 511, 511, 511, 510, 510, 510, 510, 510, 510, 509, 509, 508, 1270, 1261, 1260, 1258, 1256, 1255, 1244, 1244, 1241, 1240, 1238, 1235, 1233, 1228, 1224, 1222, 1218, 1207, 1204, 1203, 806, 806, 805, 801, 800, 799, 799, 798, 796, 794, 794, 790, 786, 786, 785, 784, 783, 783, 783, 781, 618, 617, 617, 617, 615, 615, 614, 614, 614, 614, 613, 613, 612, 612, 611, 611, 611, 611, 610, 609, 508, 507, 506, 504, 504, 503, 503, 502, 502, 501, 501, 500, 500, 500, 500, 499, 499, 499, 499, 499, 1202, 1197, 1196, 1194, 1192, 1191, 1183, 1181, 1177, 1174, 781, 780, 780, 779, 778, 778, 777, 777, 776, 771, 608, 608, 608, 607, 607, 607, 607, 606, 604, 604, 498, 498, 498, 498, 497, 496, 496, 495, 495, 494];

    let firstnames: Vec<u32> = firstnames2003.chain(firstnames2004).chain(firstnames2005).chain(firstnames2006).map(|c| c.to_digit(36).unwrap() - 10).collect();
    let firstnameweights: Vec<u32> = firstnames2003weights.into_iter().chain(firstnames2004weights.into_iter()).chain(firstnames2005weights.into_iter()).chain(firstnames2006weights.into_iter()).collect();
    
    let first_name_distribution = WeightedIndex::new(firstnameweights).unwrap();
    let last_name_distribution = WeightedIndex::new(lastnameweights).unwrap();

    let mut rng = thread_rng();

    return [(); 12000].iter()
        .fold(
            [[0; 26]; 26],
            |mut acc, _| {acc[firstnames[first_name_distribution.sample(&mut rng)] as usize][lastnames[last_name_distribution.sample(&mut rng)] as usize] += 1; acc})
        .into_iter()
        .enumerate()
        .flat_map(|first_initial| {
            let first_c = char::from_digit(first_initial.0 as u32 + 10, 36).unwrap();
            return first_initial.1.into_iter().enumerate()
                .flat_map(move |second_initial| {
                    let initial = format!("{}{}", first_c, char::from_digit(second_initial.0 as u32 + 10, 36).unwrap());
                    return (1..=second_initial.1).map(move |i| {assert!(i <= 999); format!("{}{:03}", initial, i)});
                })
        }).collect();
}
