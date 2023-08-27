// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BattleUnit
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub struct BattleUnit
  {
    pub Uattacker: i32;
    pub Coordinate UCanRetreat;
    pub UNr: i32;
    pub URetreat: i32;
    pub URetreatMode: i32;
    pub UApMoveCost: i32;
    pub UMaxApToSpend: i32;
    pub UApSpend: i32;
    pub UParticipated: i32;
    pub UDead: i32;
    pub UDefIntercept: bool;
    pub USupportInterceptFire: bool;
    pub UPanicked: bool;
    pub UParadropper: bool;
    pub float UStaffMod;
    pub float UStaffXpMod;
    pub UShatter: bool;
    pub UBreaks: bool;
    pub UAA: i32;
    pub URetreated: i32;
    pub ULos: Vec<i32>;
    pub UinitialRdn: i32;
    pub previewInfoLevel: i32;
    pub UDice: i32;
    pub UpartialAttack: bool;
    pub Coordinate UVisibility;
    pub USetToSpottedAtEnd: bool;
    pub float UepEpMaxMod;
  }
}
