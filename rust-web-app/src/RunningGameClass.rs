// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RunningGameClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;

namespace WindowsApplication1
{
  pub class RunningGameClass
  {
    pub gameInstanceID: i32;
    pub pairedGameID: i32;
    pub playerToUserName: String;
    pub Player1UserName: String;
    pub Player2UserName: String;
    pub TurnNo: i32;
    pub GameName: String;
    pub DateTime DateIssued;
    pub DateTime lastUploaded;
    pub GameOver: bool;
    pub float Player1Losses;
    pub float Player2Losses;
    pub Viewed: i32;
    pub ArmyName: String;
    pub AccepterArmyName: String;
    pub MiscData: String;
    pub DateTime lastTurnUploaded;
    pub Resigned: i32;
    pub Tournament: i32;
    pub TimeLeftPlayer1: i32;
    pub TimeLeftPlayer2: i32;
    pub LosingSide: i32;
  }
}
