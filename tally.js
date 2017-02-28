var scoreToTally = function(score){
    var tallys = {};
    tallys[1] = 'a'; 
    tallys[2] = 'b'; 
    tallys[3] = 'c'; 
    tallys[4] = 'd'; 
    tallys[5] = 'e'; 

    var retstr = '';
    var val = score;

    
    while (true) {
        val = val - 5;
        
        if (val < 0) {
            retstr += tallys[5+val];
            break;
        } else {
            retstr += "e <br>";
        }

        if (val == 0) { break; }
    }

    return retstr;
}

Test.assertEquals(scoreToTally(3),'c', "Should return 'c'");
Test.assertEquals(scoreToTally(10),'e <br>e <br>', "Should return 'e <br>e <br>'");
Test.assertEquals(scoreToTally(9),'e <br>d', "Should return 'e <br>d'");