// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.Individual
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub struct Individual
  {
    pub IUnr: i32;
    pub IUlistNr: i32;
    pub ISFNr: i32;
    pub IRdn: i32;
    pub IMor: i32;
    pub IXp: i32;
    pub IEntrench: i32;
    pub IRdnInitial: i32;
    pub IMorInitial: i32;
    pub IXpInitial: i32;
    pub IEntrenchInitial: i32;
    pub ILastAttacked: i32;
    pub ILastHit: i32;
    pub ILastAttackDone: i32;
    pub IBreakTrough: i32;
    pub IInitThrow: i32;
    pub IID: i32;
    pub IHistoricInit: Vec<i32>;
    pub IHistoricState: Vec<String>;
    pub IHistoricState2: Vec<String>;
    pub IRetreat: i32;
    pub IRetreatMode: i32;
    pub IRetreated: i32;
    pub IOutOfAp: i32;
    pub IParadropper: bool;
    pub float IAttackMod;
    pub IPreventPointsGiven: i32;
    pub IPreventPointsUsed: i32;
    pub IKilled: i32;
    pub IAttacker: i32;
    pub AttackCount: i32;
    pub AttackCountAir: i32;
    pub AttackCountLand: i32;
    pub AttackedCount: i32;
    pub ISuccesfullAttack: i32;
    pub IDammageDone: i32;
    pub IDisruption: i32;
    pub IBattleRounds: i32;
    pub ICounterAttacks: i32;
    pub ILastTargeted: i32;
    pub INoRetreat: bool;
    pub ICapitulate: bool;
    pub IBroken: bool;
    pub IHp: i32;
    pub IAA: i32;
    pub ISFType: i32;
    pub ISurrenderTestDone: bool;
    pub Istockpile: bool;
    pub float ILisFuelMod;
    pub float ILisAmmoMod;
    pub SimpleList IunitFeat;
    pub IunitFeatStart: i32;
    pub IunitFeatDeadRound: i32;
    pub float IdirectMod;
    pub IdirectFire: bool;
    pub float[] IdirectModDef;
    pub IdirectFireDef: Vec<bool>;
    pub float IcoverPoints;
    pub IvisibleFromRound: i32;
    pub ItotalKills: i32;
    pub ItotalKillsPowerPoints: i32;
    pub ItotalHits: i32;
    pub previewCoverPoints: i32;
    pub RoundsOfAttack: i32;
    pub RoundsOfAirAttack: i32;
    pub RoundsOfLandAttack: i32;
    pub IleftOutOfPartialAttack: bool;
  }
}
