// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NewEnums
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;

namespace WindowsApplication1
{
  [StandardModule]
  pub sealed class NewEnums
  {
    pub enum LibVarType
    {
      General,
      Landscape,
      Road,
      River,
      Hex,
      SFtype,
      LocationType,
      HistoricalUnit,
      HistoricalUnitModel,
      Officer,
      People,
      Regime,
    }

    pub enum LibVarValueType
    {
      Number,
      Text,
      RoadId,
      LandscapeId,
      RiverId,
      DateString,
      SFTypeId,
      HistoricalUnitId,
      HistoricalUnitModelId,
      OfficerId,
      PeopleId,
      RegimeId,
      YesNo,
      LocationTypeId,
      SmallGfxId,
      EventPicId,
      ActionCardId,
    }

    pub enum LibFileType
    {
      None,
      ImportTroopsInHistoricalEditor,
      ImportCardsInOfficerEditor,
      LoadTroops,
      LoadHistoricals,
      LoadOfficers,
      LoadOfficerCards,
      LoadEvents,
      LoadMap,
    }
  }
}
