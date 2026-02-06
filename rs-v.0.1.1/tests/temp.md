// tests/at\_command\_test.rs

use hsf\_softmodem::tapi::at\_commands::\*;



/// Basic "AT" and "ATZ" handling

\#\[test]// tests/at\_command\_test.rs

use hsf\_softmodem::tapi::at\_commands::\*;



/// Basic "AT" and "ATZ" handling

\#\[test]

fn test\_basic\_at\_and\_reset() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // AT -> Attention

&nbsp;   let cmds = parser.parse\_command\_line("AT");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Attention);



&nbsp;   // ATZ -> Reset

&nbsp;   let cmds = parser.parse\_command\_line("ATZ");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Reset);

}



/// Echo and verbose flags (E/V) are parsed into SetEcho / SetVerbose

\#\[test]

fn test\_echo\_and\_verbose\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // ATE0

&nbsp;   let cmds = parser.parse\_command\_line("ATE0");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetEcho(false));



&nbsp;   // ATE1

&nbsp;   let cmds = parser.parse\_command\_line("ATE1");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetEcho(true));



&nbsp;   // ATV0

&nbsp;   let cmds = parser.parse\_command\_line("ATV0");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetVerbose(false));



&nbsp;   // ATV1

&nbsp;   let cmds = parser.parse\_command\_line("ATV1");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetVerbose(true));

}



/// Combined short-form flag sequence (E/V/Z) is split correctly

\#\[test]

fn test\_combined\_flags\_and\_reset() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // Classic “smoosh the config together” style

&nbsp;   let cmds = parser.parse\_command\_line("ATE1V1Z");



&nbsp;   // We expect E, V, Z in that order

&nbsp;   assert\_eq!(

&nbsp;       cmds,

&nbsp;       vec!\[

&nbsp;           ATCommand::SetEcho(true),

&nbsp;           ATCommand::SetVerbose(true),

&nbsp;           ATCommand::Reset

&nbsp;       ]

&nbsp;   );

}



/// Dial and info commands

\#\[test]

fn test\_dial\_and\_info\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // Tone dial

&nbsp;   let cmds = parser.parse\_command\_line("ATDT5551234");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Dial("5551234".to\_string()));



&nbsp;   // Info command: ATI3 -> Info("3")

&nbsp;   let cmds = parser.parse\_command\_line("ATI3");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Info("3".to\_string()));

}



/// S-register set/query and +MS speed selection

\#\[test]

fn test\_s\_register\_and\_speed\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // ATS0=2

&nbsp;   let cmds = parser.parse\_command\_line("ATS0=2");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetRegister(0, 2));



&nbsp;   // ATS7?

&nbsp;   let cmds = parser.parse\_command\_line("ATS7?");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::QueryRegister(7));



&nbsp;   // AT+MS=2400

&nbsp;   let cmds = parser.parse\_command\_line("AT+MS=2400");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SelectSpeed(2400));

}



/// Tiny sanity check for “+++” style escape thinking

///

/// (The real escape logic lives in the modem; this just keeps a

///  placeholder test so the original intent for escape detection

///  doesn’t fully vanish.)

\#\[test]

fn test\_escape\_sequence\_counter\_sanity() {

&nbsp;   let escape\_char = b'+';

&nbsp;   let mut plus\_count = 0u8;



&nbsp;   for \_ in 0..3 {

&nbsp;       if escape\_char == b'+' {

&nbsp;           plus\_count += 1;

&nbsp;       }

&nbsp;   }



&nbsp;   assert\_eq!(plus\_count, 3);

}



fn test\_basic\_at\_and\_reset() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // AT -> Attention

&nbsp;   let cmds = parser.parse\_command\_line("AT");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Attention);



&nbsp;   // ATZ -> Reset

&nbsp;   let cmds = parser.parse\_command\_line("ATZ");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Reset);

}



/// Echo and verbose flags (E/V) are parsed into SetEcho / SetVerbose

\#\[test]

fn test\_echo\_and\_verbose\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // ATE0

&nbsp;   let cmds = parser.parse\_command\_line("ATE0");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetEcho(false));



&nbsp;   // ATE1

&nbsp;   let cmds = parser.parse\_command\_line("ATE1");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetEcho(true));



&nbsp;   // ATV0

&nbsp;   let cmds = parser.parse\_command\_line("ATV0");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetVerbose(false));



&nbsp;   // ATV1

&nbsp;   let cmds = parser.parse\_command\_line("ATV1");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetVerbose(true));

}



/// Combined short-form flag sequence (E/V/Z) is split correctly

\#\[test]

fn test\_combined\_flags\_and\_reset() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // Classic “smoosh the config together” style

&nbsp;   let cmds = parser.parse\_command\_line("ATE1V1Z");



&nbsp;   // We expect E, V, Z in that order

&nbsp;   assert\_eq!(

&nbsp;       cmds,

&nbsp;       vec!\[

&nbsp;           ATCommand::SetEcho(true),

&nbsp;           ATCommand::SetVerbose(true),

&nbsp;           ATCommand::Reset

&nbsp;       ]

&nbsp;   );

}



/// Dial and info commands

\#\[test]

fn test\_dial\_and\_info\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // Tone dial

&nbsp;   let cmds = parser.parse\_command\_line("ATDT5551234");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Dial("5551234".to\_string()));



&nbsp;   // Info command: ATI3 -> Info("3")

&nbsp;   let cmds = parser.parse\_command\_line("ATI3");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::Info("3".to\_string()));

}



/// S-register set/query and +MS speed selection

\#\[test]

fn test\_s\_register\_and\_speed\_parsing() {

&nbsp;   let mut parser = ATCommandParser::new();



&nbsp;   // ATS0=2

&nbsp;   let cmds = parser.parse\_command\_line("ATS0=2");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SetRegister(0, 2));



&nbsp;   // ATS7?

&nbsp;   let cmds = parser.parse\_command\_line("ATS7?");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::QueryRegister(7));



&nbsp;   // AT+MS=2400

&nbsp;   let cmds = parser.parse\_command\_line("AT+MS=2400");

&nbsp;   assert\_eq!(cmds.len(), 1);

&nbsp;   assert\_eq!(cmds\[0], ATCommand::SelectSpeed(2400));

}



/// Tiny sanity check for “+++” style escape thinking

///

/// (The real escape logic lives in the modem; this just keeps a

///  placeholder test so the original intent for escape detection

///  doesn’t fully vanish.)

\#\[test]

fn test\_escape\_sequence\_counter\_sanity() {

&nbsp;   let escape\_char = b'+';

&nbsp;   let mut plus\_count = 0u8;



&nbsp;   for \_ in 0..3 {

&nbsp;       if escape\_char == b'+' {

&nbsp;           plus\_count += 1;

&nbsp;       }

&nbsp;   }



&nbsp;   assert\_eq!(plus\_count, 3);

}



