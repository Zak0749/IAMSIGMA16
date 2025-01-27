start
     jal R13,printboard[R0]

main
     ; does turn
     jal R13,readnum[R0]
     load R7,turn           ; load the turn

     lea R1,46
     load R2,board[R6]
     cmp R1,R2
     jumpne invalid

     store R7,board[R6]
     jal R13,printboard[R0]

     lea R1,0
     lea R2,1
     lea R3,2
     jal R13,checkrun[R0]

     lea R1,3
     lea R2,4
     lea R3,5
     jal R13,checkrun[R0]

     lea R1,6
     lea R2,7
     lea R3,8
     jal R13,checkrun[R0]

     lea R1,0
     lea R2,3
     lea R3,6
     jal R13,checkrun[R0]

     lea R1,1
     lea R2,4
     lea R3,7
     jal R13,checkrun[R0]

     lea R1,2
     lea R2,5
     lea R3,8
     jal R13,checkrun[R0]

     lea R1,0
     lea R2,4
     lea R3,8
     jal R13,checkrun[R0]

     lea R1,2
     lea R2,4
     lea R3,6
     jal R13,checkrun[R0]

     ; check if the game is draw
     load R1,num
     lea R2,1
     add R1,R2,R1
     lea R3,9
     cmp R3,R1
     jumpeq end_draw
     store R1,num

     ; switch turn
     load R7,turn
     lea R8,79
     cmp R7,R8
     jumpeq player2

player1 lea R7,79
     store R7,turn
     jump main
player2 lea R7,88
     store R7,turn
     jump main
invalid
     lea R4,2
     lea R5,15
     lea R6,invalid_message
     trap R4,R6,R5
     jump main

checkrun
     load R4,board[R1]
     load R5,board[R2]
     load R6,board[R3]
     lea R7,46

     cmp R4,R7
     jumpeq notrun

     cmp R4,R5
     jumpne notrun
     cmp R5,R6
     jumpne notrun
     jump end_won


notrun
     jump 0[R13]


readnum
     lea R4,0
     store R4,buffer[R0]   ; clear the buffer

     lea   R1,1[R0]         ; trap code for non-blocking read
     lea   R2,buffer[R0]    ; place to put lastInput
     load  R3,bufsize[R0]   ; only read one character
     trap  R1,R2,R3         ; read the input and store at buffer memory adress

     load R4,buffer         ; load the character

     cmp     R4,R0      ; compare R1 with R2
     jumpeq  readnum[R0]    ; if R1 < R2 then goto yes

     lea R5,49              ; load the character '1'
     sub R6,R4,R5           ; subtract the character '1' from the input to get 1 = 0, 2 = 1 etc

     jump 0[R13]

printboard
          lea R4,2         ; code for outputting
          lea R5,1          ; to use for newline
          lea R6,newline        ; newline char

          trap  R4,R6,R5  ; output newline

          lea R1,0          ; start at the first row

          lea R2,3          ; go to the next row
          lea R3,board[R1] ; load the first row
          trap   R4,R3,R2  ; output the first row

          trap  R4,R6,R5  ; output newline

          add R1,R1,R2
          lea R3,board[R1]
          trap   R4,R3,R2

          trap  R4,R6,R5  ; output newline

          add R1,R1,R2
          lea R3,board[R1]
          trap   R4,R3,R2

          trap  R4,R6,R5  ; output newline

          jump 0[R13]

end_draw
     lea R4,2
     lea R5,7
     lea R6,draw_message
     trap R4,R6,R5
     trap  R0,R0,R0         ; terminate program

end_won          ; terminate program
     load R7,turn
     lea R8,88

     cmp R7,R8
     jumpeq end_player1_won
     jump end_player2_won

end_player1_won
     lea R4,2
     lea R5,13
     lea R6,player1_won_message
     trap R4,R6,R5
     trap  R0,R0,R0         ; terminate program

end_player2_won
     lea R4,2
     lea R5,13
     lea R6,player2_won_message
     trap R4,R6,R5           ; output the message
     trap  R0,R0,R0         ; terminate program

bufsize  data  1            ; size of the buffer; don't read more than this
buffer data  0
board data 46
      data 46
      data 46
      data 46
      data 46
      data 46
      data 46
      data 46
      data 46
newline data 10
num data 0
turn data 88

invalid_message data 73
                 data 110
                 data 118
                 data 97
                 data 108
                 data 105
                 data 100
                 data 32
                 data 109
                 data 111
                 data 118
                 data 101
                 data 46
                 data 10
                 data 0

draw_message data 68
              data 114
              data 97
              data 119
              data 46
              data 10
              data 0

player1_won_message data 80
                    data 108
                    data 97
                    data 121
                    data 101
                    data 114
                    data 32
                    data 49
                    data 32
                    data 119
                    data 111
                    data 110
                    data 46
                    data 10
                    data 0

player2_won_message data 80
                    data 108
                    data 97
                    data 121
                    data 101
                    data 114
                    data 32
                    data 50
                    data 32
                    data 119
                    data 111
                    data 110
                    data 46
                    data 10
                    data 0
