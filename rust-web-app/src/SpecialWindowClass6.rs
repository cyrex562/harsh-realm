// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass6
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SpecialWindowClass6 : WindowClass
  {
     ListClass ListObj;
     ListClass List2Obj;
     listId: i32;
     list2Id: i32;
     useWidth: i32;
     useHeight: i32;
     slotAssetPresentation: i32;
     slotPerks: i32;
     slotPreviewAssetLog: i32;
     slotHexPerk: i32;
     slotPaid: i32;
     slotHexNames: i32;
     slotLandscape: i32;
     slotAssetLog: i32;
     slotConstruction: i32;
     slotZones: i32;
     slotRegKey: i32;
     slotDetail: i32;
     slotRegReg: i32;
     slotRegZoneKeys: i32;
     slotItemType: i32;
     slotRegimes: i32;
     slotZoneKeys: i32;
     slotAssetTypes: i32;
     slotAssets: i32;
     slotCharacter: i32;
     slotPortrait: i32;
     slotModel: i32;
     slotModelType: i32;
     slotModelTech: i32;
     slotTechType: i32;
     slotModelStatName: i32;
     slotModelStatBefore: i32;
     slotModelStat: i32;
     slotQuality: i32;
     slotChoice: i32;
     slotLog: i32;
     slotOldChar: i32;
     slotProfileDocs: i32;
     slotSkills: i32;
     slotSkillTypes: i32;
     slotTraits: i32;
     slotFactionProfile: i32;
     slotFactions: i32;
     slotFeatFx: i32;
     slotFeats: i32;
     slotFeatTypes: i32;
     slotFlagInstructions: i32;
     slotFlags: i32;
     slotCulture: i32;
     int[] assetButton;
     assetButtonCounter: i32;
     int[] assetButtonData;
     StringListClass leaderStr;
     SimpleList ColList;
     maxSelection: i32;
     curSelection: i32;
     but0id: i32;
     but1id: i32;
     but2id: i32;
     but3id: i32;
     but4id: i32;
     groupNames: Vec<String>;
     colNames: Vec<String>;
     FIX_KEY_RELATION: i32;
     FIX_KEY_CAPLEVEL: i32;
     FIX_KEY_SUITABILITY: i32;
     FIX_KEY_LOYALTY: i32;
     FIX_KEY_SENIORITY: i32;
     FIX_KEY_TR: i32;
     FIX_KEY_CR: i32;
     FIX_KEY_IPR: i32;
     FIX_KEY_FACTION: i32;
     FIX_KEY_AGE: i32;
     FIX_KEY_NATRELPOINT: i32;
     FIX_JOB_RESERVE: i32;
     FIX_JOB_OHQ: i32;
     FIX_JOB_SHQ: i32;
     FIX_JOB_DIRECTOR: i32;
     FIX_JOB_ADVISOR: i32;
     FIX_JOB_SECRETARY: i32;
     FIX_JOB_GOVERNOR: i32;
     FIX_STAT_STR: i32;
     FIX_STAT_WAR: i32;
     FIX_STAT_CHA: i32;
     FIX_STAT_INT: i32;
     FIX_STAT_WIL: i32;
     int[] FIX_SKILL;
     FIX_PERS_EMOT: i32;
     FIX_PERS_HERO: i32;
     FIX_PERS_AMB: i32;
     FIX_PERS_AUTH: i32;
     FIX_PERS_EGO: i32;
     FIX_PERS_COR: i32;
     FIX_PERS_THEFT: i32;
     int[] tempColId;

    pub fn Dispose() => base.Dispose();

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      return base.HandleMouseMove(x, y);
    }

    pub SpecialWindowClass6( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
      : base( tGame, tUseWidth, tUseHeight, 8)
    {
      self.ListObj = ListClass::new();
      self.List2Obj = ListClass::new();
      self.assetButton = new int[600];
      self.assetButtonCounter = -1;
      self.assetButtonData = new int[600];
      self.maxSelection = 0;
      self.curSelection = 0;
      self.groupNames = new string[100];
      self.colNames = new string[1000];
      self.FIX_KEY_RELATION = 101;
      self.FIX_KEY_CAPLEVEL = 102;
      self.FIX_KEY_SUITABILITY = 103;
      self.FIX_KEY_LOYALTY = 104;
      self.FIX_KEY_SENIORITY = 105;
      self.FIX_KEY_TR = 106;
      self.FIX_KEY_CR = 107;
      self.FIX_KEY_IPR = 108;
      self.FIX_KEY_FACTION = 109;
      self.FIX_KEY_AGE = 110;
      self.FIX_KEY_NATRELPOINT = 111;
      self.FIX_JOB_RESERVE = 1;
      self.FIX_JOB_OHQ = 3;
      self.FIX_JOB_SHQ = 4;
      self.FIX_JOB_DIRECTOR = 5;
      self.FIX_JOB_ADVISOR = 6;
      self.FIX_JOB_SECRETARY = 8;
      self.FIX_JOB_GOVERNOR = 10;
      self.FIX_STAT_STR = 401;
      self.FIX_STAT_WAR = 402;
      self.FIX_STAT_CHA = 403;
      self.FIX_STAT_INT = 404;
      self.FIX_STAT_WIL = 405;
      self.FIX_SKILL = new int[300];
      self.FIX_PERS_EMOT = 501;
      self.FIX_PERS_HERO = 502;
      self.FIX_PERS_AMB = 503;
      self.FIX_PERS_AUTH = 504;
      self.FIX_PERS_EGO = 505;
      self.FIX_PERS_COR = 506;
      self.FIX_PERS_THEFT = 507;
      self.tempColId = new int[1000];
      self.useWidth = tUseWidth;
      self.useHeight = tUseHeight;
      self.colNames[self.FIX_KEY_RELATION] = "Relation";
      self.colNames[self.FIX_KEY_CAPLEVEL] = "Capability Lvl";
      self.colNames[self.FIX_KEY_SUITABILITY] = "Suitability";
      self.colNames[self.FIX_KEY_LOYALTY] = "Loyalty";
      self.colNames[self.FIX_KEY_SENIORITY] = "Seniority";
      self.colNames[self.FIX_KEY_TR] = "Technical Rating";
      self.colNames[self.FIX_KEY_CR] = "Command Rating";
      self.colNames[self.FIX_KEY_IPR] = "Interpersonal Rating";
      self.colNames[self.FIX_KEY_FACTION] = "Faction";
      self.colNames[self.FIX_KEY_AGE] = "Age";
      self.colNames[self.FIX_KEY_NATRELPOINT] = "Natural Relation Point";
      self.colNames[self.FIX_JOB_ADVISOR] = "Advisor";
      self.colNames[self.FIX_JOB_DIRECTOR] = "Director";
      self.colNames[self.FIX_JOB_GOVERNOR] = "Governor";
      self.colNames[self.FIX_JOB_OHQ] = "OHQ Commander";
      self.colNames[self.FIX_JOB_RESERVE] = "Reserve Pool";
      self.colNames[self.FIX_JOB_SECRETARY] = "Secretary";
      self.colNames[self.FIX_JOB_SHQ] = "SHQ Commander";
      self.colNames[self.FIX_STAT_CHA] = "Charisma";
      self.colNames[self.FIX_STAT_INT] = "Intelligence";
      self.colNames[self.FIX_STAT_STR] = "Strength";
      self.colNames[self.FIX_STAT_WAR] = "War";
      self.colNames[self.FIX_STAT_WIL] = "Willpower";
      self.colNames[self.FIX_PERS_AMB] = "Ambition";
      self.colNames[self.FIX_PERS_AUTH] = "Authority";
      self.colNames[self.FIX_PERS_COR] = "Corruption";
      self.colNames[self.FIX_PERS_EGO] = "Egoism";
      self.colNames[self.FIX_PERS_EMOT] = "Emotional";
      self.colNames[self.FIX_PERS_HERO] = "Heroic";
      self.colNames[self.FIX_PERS_THEFT] = "Theft";
      self.groupNames[1] = "Job Type";
      self.groupNames[2] = "Key Info";
      self.groupNames[3] = "Stats";
      self.groupNames[4] = "Technical Skills";
      self.groupNames[5] = "Command Skills";
      self.groupNames[6] = "Interpersonal Skills";
      self.groupNames[7] = "Non-Grouped Skills";
      self.groupNames[8] = "Personality";
      self.groupNames[9] = "Aux. Info";
      libName: String = "SE_Data";
      self.slotCharacter = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.slotRegKey = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      self.slotFlagInstructions = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 169, 0, 0));
      self.slotFlags = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 168, 0, 0));
      self.slotRegimes = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 143, 0, 0));
      self.slotCulture = tGame.HandyFunctionsObj.GetStringListByID(tGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      self.slotOldChar = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 365, 0, 0));
      self.slotTraits = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 197, 0, 0));
      self.slotSkills = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 199, 0, 0));
      self.slotLog = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 198, 0, 0));
      self.slotSkillTypes = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 203, 0, 0));
      self.slotProfileDocs = tGame.HandyFunctionsObj.GetStringListByID(tGame.EventRelatedObj.CheckStringlistID(libName, 258, 0, 0));
      self.slotFeats = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 376, 0, 0));
      self.slotFeatTypes = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 377, 0, 0));
      self.slotFeatFx = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 378, 0, 0));
      self.slotFactions = tGame.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 200, 0, 0));
      self.slotFactionProfile = tGame.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 202, 0, 0));
      self.ColList = SimpleList::new();
      let mut length: i32 = self.game.Data.StringListObj[self.slotSkillTypes].Length;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        let mut index2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotSkillTypes].Data[index1, 0]));
        str: String = self.game.Data.StringListObj[self.slotSkillTypes].Data[index1, 1];
        let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotSkillTypes].Data[index1, 12]));
        if (Operators.CompareString(str.ToLower(), "n/a", false) != 0)
        {
          let mut tid: i32 = 200 + index2;
          self.FIX_SKILL[index2] = tid;
          self.colNames[tid] = str;
          if (num == 2)
            self.ColList.Add(tid, 1, 4);
          if (num == 3)
            self.ColList.Add(tid, 1, 5);
          if (num == 1)
            self.ColList.Add(tid, 1, 6);
          if (num == 0)
            self.ColList.Add(tid, 1, 7);
        }
      }
      self.assetButtonCounter = -1;
      if (Information.IsNothing( self.game.EditObj.se1_leaderColumns))
      {
        self.game.EditObj.se1_leaderColumns = SimpleList::new();
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_KEY_RELATION, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_KEY_NATRELPOINT, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_ADVISOR, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_DIRECTOR, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_GOVERNOR, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_OHQ, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_RESERVE, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_SECRETARY, 0);
        self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_SHQ, 0);
      }
      if (self.game.EditObj.se1_leaderGroup < 1)
        self.game.EditObj.se1_leaderGroup = 1;
      self.ColList.Add(self.FIX_JOB_ADVISOR, 1, 1);
      self.ColList.Add(self.FIX_JOB_DIRECTOR, 1, 1);
      self.ColList.Add(self.FIX_JOB_GOVERNOR, 1, 1);
      self.ColList.Add(self.FIX_JOB_OHQ, 1, 1);
      self.ColList.Add(self.FIX_JOB_RESERVE, 1, 1);
      self.ColList.Add(self.FIX_JOB_SECRETARY, 1, 1);
      self.ColList.Add(self.FIX_JOB_SHQ, 1, 1);
      self.ColList.Add(self.FIX_KEY_RELATION, 1, 2);
      self.ColList.Add(self.FIX_KEY_NATRELPOINT, 1, 2);
      self.ColList.Add(self.FIX_KEY_CAPLEVEL, 1, 2);
      self.ColList.Add(self.FIX_KEY_SUITABILITY, 1, 2);
      self.ColList.Add(self.FIX_KEY_LOYALTY, 1, 2);
      self.ColList.Add(self.FIX_KEY_SENIORITY, 1, 2);
      self.ColList.Add(self.FIX_KEY_TR, 1, 2);
      self.ColList.Add(self.FIX_KEY_CR, 1, 2);
      self.ColList.Add(self.FIX_KEY_IPR, 1, 2);
      self.ColList.Add(self.FIX_KEY_FACTION, 1, 2);
      self.ColList.Add(self.FIX_KEY_AGE, 1, 2);
      self.ColList.Add(self.FIX_STAT_STR, 1, 3);
      self.ColList.Add(self.FIX_STAT_WAR, 1, 3);
      self.ColList.Add(self.FIX_STAT_CHA, 1, 3);
      self.ColList.Add(self.FIX_STAT_INT, 1, 3);
      self.ColList.Add(self.FIX_STAT_WIL, 1, 3);
      self.ColList.Add(self.FIX_PERS_EMOT, 1, 8);
      self.ColList.Add(self.FIX_PERS_HERO, 1, 8);
      self.ColList.Add(self.FIX_PERS_AMB, 1, 8);
      self.ColList.Add(self.FIX_PERS_AUTH, 1, 8);
      self.ColList.Add(self.FIX_PERS_EGO, 1, 8);
      self.ColList.Add(self.FIX_PERS_COR, 1, 8);
      self.ColList.Add(self.FIX_PERS_THEFT, 1, 8);
      self.ReCalculate();
      self.dostuff();
    }

    pub fn ReCalculate()
    {
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      let mut counter1: i32 = self.game.EditObj.se1_leaderColumns.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
        self.game.EditObj.se1_leaderColumns.Data5[index] = self.game.EditObj.se1_leaderColumns.Id[index];
      self.game.EditObj.se1_leaderColumns.SortOnData5();
      self.leaderStr = new StringListClass(0);
      self.maxSelection =  Math.Round(Conversion.Val( ( (self.useWidth - 550) / 50.0)));
      self.curSelection = 0;
      self.leaderStr.AddCol(-1, "Char ID");
      let mut counter2: i32 = self.game.EditObj.se1_leaderColumns.Counter;
      for (let mut index: i32 = 0; index <= counter2; index += 1)
      {
        colName: String = self.colNames[self.game.EditObj.se1_leaderColumns.Id[index]];
        if (self.game.EditObj.se1_leaderColumns.Id[index] > 100)
        {
          this += 1.curSelection;
          self.leaderStr.AddCol(self.curSelection, colName);
          self.tempColId[self.curSelection] = self.game.EditObj.se1_leaderColumns.Id[index];
        }
      }
      let mut num1: i32 =  Math.Round(Conversion.Val( self.game.Data.StringListObj[self.slotSkillTypes].GetHighestValue(0)));
      let mut length1: i32 = self.game.Data.StringListObj[self.slotCharacter].Length;
      for (let mut index1: i32 = 0; index1 <= length1; index1 += 1)
      {
        if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 5])) == id)
        {
          let mut num2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 0]));
          let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 6]));
          bool flag1 = false;
          let mut counter3: i32 = self.game.EditObj.se1_leaderColumns.Counter;
          for (let mut index2: i32 = 0; index2 <= counter3; index2 += 1)
          {
            let mut num4: i32 = self.game.EditObj.se1_leaderColumns.Id[index2];
            if (num4 < 20 && num3 == num4)
              flag1 = true;
          }
          if (flag1)
          {
            self.leaderStr.AddRow(self.leaderStr.Length);
            let mut length2: i32 = self.leaderStr.Length;
            self.leaderStr.Data[length2, 0] = num2.ToString();
            let mut num5: i32 = -1;
            let mut counter4: i32 = self.game.EditObj.se1_leaderColumns.Counter;
            for (let mut index3: i32 = 0; index3 <= counter4; index3 += 1)
            {
              bool flag2 = false;
              let mut num6: i32 = self.game.EditObj.se1_leaderColumns.Id[index3];
              if (num6 >= 100)
              {
                num5 += 1;
                let mut num7: i32 = self.game.EditObj.se1_leaderColumns.Weight[index3];
                str1: String = "";
                str2: String = "";
                nr: i32;
                if (num6 == self.FIX_KEY_RELATION)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 20]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_NATRELPOINT)
                {
                  let mut charId: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 0]));
                  FunctionFeedback naturalRelationPoint = self.game.EventRelatedObj.Helper_GetCharacterNaturalRelationPoint(id, charId);
                  nr = naturalRelationPoint.value;
                  str1 = nr.ToString();
                  str2 = naturalRelationPoint.texty;
                }
                if (num6 == self.FIX_KEY_CAPLEVEL)
                {
                  flag2 = true;
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 1]));
                  str1 = nr.ToString();
                  if (nr > 0)
                    str1 = self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                }
                if (num6 == self.FIX_KEY_SUITABILITY)
                {
                  nr = self.game.EventRelatedObj.Helper_GetCharacterSuitabilityRating( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 0])),  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 6])),  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 7])));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_LOYALTY)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 19]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_SENIORITY)
                {
                  nr = self.game.EventRelatedObj.Helper_GetCharacterSeniorityRanking(id, num2);
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_TR)
                {
                  nr = self.game.EventRelatedObj.Helper_GetCharacterTechnicalRating(num2);
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_CR)
                {
                  nr = self.game.EventRelatedObj.Helper_GetCharacterCommandRating(num2);
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_IPR)
                {
                  nr = self.game.EventRelatedObj.Helper_GetCharacterInterpersonalRating(num2);
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_KEY_FACTION)
                {
                  flag2 = true;
                  let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].GetData(0, num2, 13)));
                  str1 = idValue <= 0 ? "-" : self.game.Data.StringListObj[self.slotFactions].GetData(0, idValue, 10);
                }
                if (num6 == self.FIX_KEY_AGE)
                {
                  nr = self.game.Data.Round -  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 16]));
                  nr =  Math.Round(Math.Floor( nr / 6.0));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_STAT_STR)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 35]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_STAT_WAR)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 36]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_STAT_CHA)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 37]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_STAT_INT)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 38]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_STAT_WIL)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 39]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_EMOT)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 45]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_HERO)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 46]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_AMB)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 47]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_AUTH)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 48]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_EGO)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 49]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_COR)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 29]));
                  str1 = nr.ToString();
                }
                if (num6 == self.FIX_PERS_THEFT)
                {
                  nr =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].Data[index1, 28]));
                  str3: String = nr.ToString();
                  str1 = nr >= 1 ? str3 + " Cr" : "None";
                }
                if (num6 >= 200 & num6 < 400)
                {
                  str1 = "-";
                  let mut num8: i32 = num1;
                  for (let mut idValue2: i32 = 1; idValue2 <= num8; idValue2 += 1)
                  {
                    if (num6 == self.FIX_SKILL[idValue2])
                    {
                      let mut num9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotSkills].GetData2(0, num2, 1, idValue2, 2)));
                      str1 = num9.ToString();
                      if (num9 < 1)
                        str1 = "-";
                    }
                  }
                }
                self.leaderStr.Data[length2, num5 + 1] = str1;
                self.leaderStr.TempDesc[length2, num5 + 1] = str2;
                self.leaderStr.ColValueType[num5 + 1] = !flag2 ? NewEnums.LibVarValueType.Number : NewEnums.LibVarValueType.Text;
              }
            }
          }
        }
      }
      let mut curSelection: i32 = self.curSelection;
      for (let mut col: i32 = 1; col <= curSelection; col += 1)
      {
        let mut nr: i32 = self.game.EditObj.se1_leaderColumns.FindNr(self.tempColId[col]);
        if (nr > -1)
        {
          switch (self.game.EditObj.se1_leaderColumns.Weight[nr])
          {
            case 1:
              self.leaderStr.Sort(col);
              continue;
            case 2:
              self.leaderStr.Sort(col);
              self.leaderStr.Sort(col);
              continue;
            default:
              continue;
          }
        }
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff(bool crmAlreadySet = false)
    {
      SizeF sizeF1 = SizeF::new();
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      let mut turn: i32 = self.game.Data.Turn;
      let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCulture].GetData(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotRegimes].GetData(0, id, 2))), 1)));
      self.game.Data.StringListObj[self.slotFlagInstructions].SetData(0, "REGIMEID", 1, id);
      self.game.Data.StringListObj[self.slotFlagInstructions].SetData(0, "ROUND", 1, self.game.Data.Round);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.useWidth, self.useHeight, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawRepeatingBackground(g, DrawMod.TGame.BACKGROUND3MARC, 0, 0, self.useWidth, self.useHeight);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (self.listId > 0)
      {
        self.RemoveSubPart(self.listId);
        self.listId = 0;
      }
      if (self.list2Id > 0)
      {
        self.RemoveSubPart(self.list2Id);
        self.list2Id = 0;
      }
      if (self.but0id > 0)
      {
        self.RemoveSubPart(self.but0id);
        self.but0id = 0;
      }
      if (self.but1id > 0)
      {
        self.RemoveSubPart(self.but1id);
        self.but1id = 0;
      }
      if (self.but2id > 0)
      {
        self.RemoveSubPart(self.but2id);
        self.but2id = 0;
      }
      if (self.but3id > 0)
      {
        self.RemoveSubPart(self.but3id);
        self.but3id = 0;
      }
      if (self.but4id > 0)
      {
        self.RemoveSubPart(self.but4id);
        self.but4id = 0;
      }
      let mut assetButtonCounter1: i32 = self.assetButtonCounter;
      for (let mut index: i32 = 0; index <= assetButtonCounter1; index += 1)
      {
        if (self.assetButton[index] > 0)
        {
          self.RemoveSubPart(self.assetButton[index]);
          self.assetButton[index] = 0;
          self.assetButtonData[index] = 0;
        }
      }
      self.assetButtonCounter = -1;
      let mut y1: i32 = 80;
      Rectangle rectangle1 = Rectangle::new(0, y1, 220, self.useHeight);
      let mut width1: i32 = rectangle1.Width;
      let mut y2: i32 = y1;
      let mut width2: i32 = self.useWidth - width1;
      let mut height1: i32 = 50;
      Rectangle rectangle2 = Rectangle::new(width1, y2, width2, height1);
      Rectangle rectangle3 = Rectangle::new(rectangle2.Left, rectangle2.Top + rectangle2.Height, rectangle2.Width, self.useHeight - (rectangle2.Height + 200 + y1));
      Rectangle rectangle4 = Rectangle::new(rectangle3.X, rectangle3.Y + rectangle3.Height, rectangle3.Width, 200);
      DrawMod.DrawBlock( g, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, 0, 0, 0, 120);
      DrawMod.DrawBlock( g, rectangle2.X, rectangle2.Y, rectangle2.Width, rectangle2.Height, 0, 0, 0, 160);
      DrawMod.DrawBlock( g, rectangle3.X, rectangle3.Y, rectangle3.Width, rectangle3.Height, 0, 0, 0, 140);
      DrawMod.DrawBlock( g, rectangle4.X, rectangle4.Y, rectangle4.Width, rectangle4.Height, 0, 0, 0, 170);
      self.ListObj = ListClass::new();
      let mut left1: i32 = rectangle1.Left;
      let mut top1: i32 = rectangle3.Top;
      let mut num2: i32 = rectangle1.Width - 10;
      let mut tlistsize1: i32 =  Math.Round(Math.Floor(210.0 / 24.0)) - 1;
      let mut tlistselect: i32 = -1;
      let mut num3: i32 = -1;
      let mut tdata: i32 = 1;
      do
      {
        num3 += 1;
        groupName: String = self.groupNames[tdata];
        if (self.game.EditObj.se1_leaderGroup == tdata)
          tlistselect = num3;
        self.ListObj.add(groupName, tdata);
        tdata += 1;
      }
      while (tdata <= 8);
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(self.ListObj, tlistsize1, num2, tlistselect, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left1, bby: top1, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 24);
      self.listId = self.AddSubPart( tsubpart1, left1, top1, num2, (tlistsize1 + 1) * 24, 1);
      let mut num4: i32 = top1 + 210;
      self.List2Obj = ListClass::new();
      let mut tlistsize2: i32 =  Math.Round(Math.Floor( (rectangle1.Height - (num4 + 50)) / 26.0)) - 1;
      let mut num5: i32 = 1 - 1;
      self.List2Obj.add("None", -2);
      self.List2Obj.add("All", 0);
      let mut counter: i32 = self.ColList.Counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
      {
        if (self.ColList.Data1[index] == self.game.EditObj.se1_leaderGroup)
        {
          num5 += 1;
          colName: String = self.colNames[self.ColList.Id[index]];
          if (self.game.EditObj.se1_leaderColumns.FindNr(self.ColList.Id[index]) > -1)
            self.List2Obj.add(colName, self.ColList.Id[index], tbmp: BitmapStore.GetBitmap(self.game.SMALLCHAR2));
          else
            self.List2Obj.add(colName, self.ColList.Id[index], tbmp: BitmapStore.GetBitmap(self.game.SMALLCHAR1));
        }
      }
      let mut tsubpart2: SubPartClass =  new ListSubPartClass(self.List2Obj, tlistsize2, num2, -1, self.game, tShowPair: true, tValueWidth: 70, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left1, bby: num4, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 26);
      self.list2Id = self.AddSubPart( tsubpart2, left1, num4, num2, (tlistsize2 + 1) * 26, 1);
      let mut x1: i32 = rectangle2.Left + 10;
      let mut y3: i32 = rectangle2.Top + 10;
      str1: String = "Leaders";
      DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont2, x1, y3, Color.White);
      SizeF sizeF2 = g.MeasureString(str1, self.game.MarcFont2);
      let mut x2: i32 =  Math.Round( rectangle2.Left +  sizeF2.Width + 40.0);
      let mut num6: i32 = 75;
      let mut y4: i32 = rectangle2.Top + 3;
      let mut tsubpart3: SubPartClass =  new SEBigTextPartClass("Reset", "Change back to default selection of data columns", self.game.EditObj.se1_modelView == 0, num6, 44);
      self.but0id = self.AddSubPart( tsubpart3, x2, y4, num6, 44, 1);
      let mut num7: i32 = x2 + 80;
      let mut left2: i32 = rectangle3.Left;
      let mut top2: i32 = rectangle3.Top;
      let mut height2: i32 = 57;
      let mut width3: i32 = rectangle3.Width;
      let mut num8: i32 =  Math.Round(Math.Floor( (rectangle3.Height - 20) /  height2));
      let mut num9: i32 = 1 +  Math.Round(Math.Floor( (self.leaderStr.Length + 1 - 1) /  num8));
      if (num9 < self.game.EditObj.se1_leaderPage)
        self.game.EditObj.se1_leaderPage = num9;
      if (self.game.EditObj.se1_leaderPage < 1)
        self.game.EditObj.se1_leaderPage = 1;
      let mut num10: i32 = (self.game.EditObj.se1_leaderPage - 1) * num8 + 1;
      let mut num11: i32 =  Math.Round(Math.Floor( (rectangle2.Width - 400) /  num9));
      if (num11 > 100)
        num11 = 100;
      let mut num12: i32 = left2;
      let mut num13: i32 = top2;
      let mut x3: i32 = rectangle2.Right - (10 + (num11 + 4) * num9);
      let mut y5: i32 = rectangle2.Top + 3;
      let mut num14: i32 = num9;
      for (let mut index: i32 = 1; index <= num14; index += 1)
      {
        this += 1.assetButtonCounter;
        tDescript: String = index.ToString() + "/" + num9.ToString() + ". Click to view this Leader page.";
        if (self.game.EditObj.se1_modelPage == index)
          tDescript = index.ToString() + "/" + num9.ToString() + ". Currently selected Leader page";
        int[] assetButton = self.assetButton;
        let mut assetButtonCounter2: i32 = self.assetButtonCounter;
        tsubpart3 =  new SEBigTextPartClass(index.ToString(), tDescript, self.game.EditObj.se1_leaderPage == index, num11, 44);
        let mut num15: i32 = self.AddSubPart( tsubpart3, x3, y5, num11, 44, 1);
        assetButton[assetButtonCounter2] = num15;
        self.assetButtonData[self.assetButtonCounter] = 50 + index;
        x3 += num11 + 4;
      }
      let mut x1_1: i32 = num12;
      let mut y1_1: i32 = num13;
      DrawMod.DrawBlock( g, x1_1, y1_1, width3 - 10, 31, 168, 168, 168, 70);
      let mut num16: i32 = y1_1 + 32;
      let mut num17: i32 = 0;
      let mut num18: i32 = 0;
      let mut num19: i32 = 20;
      let mut num20: i32 = 100;
      if (self.curSelection > 0)
        num20 =  Math.Round(Math.Floor( (rectangle3.Width - 420) /  self.curSelection));
      if (num20 > 150)
        num20 = 150;
      let mut length: i32 = self.leaderStr.Length;
      Rectangle trect;
      Rectangle rectangle5;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        num18 += 1;
        let mut num21: i32 =  Math.Round(Conversion.Val(self.leaderStr.Data[index1, 0]));
        if (num18 >= num10 & num18 <= num10 - 1 + num8)
        {
          num17 += 1;
          let mut left3: i32 = rectangle3.Left;
          if (num21 != self.game.EditObj.se1_leaderSelected)
          {
            if (num17 % 2 == 0)
              DrawMod.DrawBlock( g, left3, num16, width3 - 10, height2 - 1, 148, 148, 148, 140);
            else
              DrawMod.DrawBlock( g, left3, num16, width3 - 10, height2 - 1, 128, 128, 128, 140);
          }
          else
            DrawMod.DrawBlock( g, left3, num16, width3 - 10, height2 - 1, 148, 218, 148, 140);
          let mut x4: i32 = left3 + 10;
          bitmap: Bitmap = (Bitmap) self.game.CustomBitmapObj.DrawLeaderPortrait(num21, 40, 56).Clone();
          if (num17 == 1)
            DrawMod.DrawTextColouredMarc( g, "PORTRAIT, NAME, JOB", self.game.MarcFont5, x4, num16 - 28, Color.White);
           let mut local1: &Graphics = &g;
           let mut local2: &Bitmap = &bitmap;
          trect = Rectangle::new(0, 0, bitmap.Width, bitmap.Height);
          let mut srcrect: &Rectangle = &trect
          rectangle5 = Rectangle::new(x4, num16, 40, 56);
          let mut destrect: &Rectangle = &rectangle5
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          bitmap.Dispose();
          bitmap = (Bitmap) null;
          rectangle5 = Rectangle::new(x4, num16, 50, height2);
          trect = rectangle5;
          self.AddMouse( trect, "", "Click to get more information on this Leader", 20000000 + num21);
          let mut x5: i32 = x4 + 50;
          tstring: String = self.game.Data.StringListObj[self.slotCharacter].GetData(0, num21, 3) + " " + self.game.Data.StringListObj[self.slotCharacter].GetData(0, num21, 4);
          characterJobTitle: String = self.game.EventRelatedObj.Helper_GetCharacterJobTitle(num21);
          DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont16, x5, num16 + num19 - 12, Color.White);
          DrawMod.DrawTextColouredMarc( g, characterJobTitle, self.game.MarcFont4, x5, num16 + num19 + 12, Color.LightGray);
          rectangle5 = Rectangle::new(x5, num16, 300, height2);
          trect = rectangle5;
          self.AddMouse( trect, "", "Click to select this row", 1000000 + num21);
          let mut x6: i32 = x5 + 300;
          let mut width4: i32 = self.leaderStr.Width;
          for (let mut index2: i32 = 1; index2 <= width4; index2 += 1)
          {
            if (index2 % 2 == 0)
              DrawMod.DrawBlock( g, x6 - 5, num16, num20, height2 - 1, 0, 0, 0, 40);
            if (self.leaderStr.ColSort[index2] == 2 & self.tempColId[index2] != 102 | self.leaderStr.ColSort[index2] == 1 & self.tempColId[index2] == 102)
              DrawMod.DrawBlock( g, x6 - 5, num16, num20, height2 - 1, 0,  byte.MaxValue, 0, 40);
            if (self.leaderStr.ColSort[index2] == 1 & self.tempColId[index2] != 102 | self.leaderStr.ColSort[index2] == 2 & self.tempColId[index2] == 102)
              DrawMod.DrawBlock( g, x6 - 5, num16, num20, height2 - 1,  byte.MaxValue, 0, 0, 40);
            if (num17 == 1)
            {
              str2: String = self.leaderStr.ColumnName[index2];
              let mut num22: i32 = Strings.InStr(str2, " ");
              if (num22 > 0)
                str2 = Strings.Left(str2, num22 - 1) + "\r\n" + Strings.Mid(str2, num22 + 1);
              for (SizeF sizeF3 = g.MeasureString(str2, self.game.MarcFont5);  sizeF3.Width >  num20; sizeF3 = g.MeasureString(str2, self.game.MarcFont5))
                str2 = Strings.Left(str2, str2.Length - 1);
              DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont5, x6, num16 - 28, Color.White);
              rectangle5 = Rectangle::new(x6, num16 - 32, num20, 32);
              trect = rectangle5;
              self.AddMouse( trect, "", "Click to sort differently", 50000 + index2);
            }
            str3: String = self.leaderStr.Data[index1, index2];
            ttext: String = self.leaderStr.TempDesc[index1, index2];
            for (SizeF sizeF4 = g.MeasureString(str3, self.game.MarcFont4);  sizeF4.Width >  num20; sizeF4 = g.MeasureString(str3, self.game.MarcFont4))
              str3 = Strings.Left(str3, str3.Length - 1);
            DrawMod.DrawTextColouredMarc( g, str3, self.game.MarcFont4, x6, num16 + num19, Color.White);
            if (ttext.Length > 1)
            {
              rectangle5 = Rectangle::new(x6 - 5, num16, num20, height2 - 1);
              trect = rectangle5;
              self.AddMouse( trect, "", ttext);
            }
            else
            {
              rectangle5 = Rectangle::new(x6 - 5, num16, num20, height2 - 1);
              trect = rectangle5;
              self.AddMouse( trect, "", "Click to select this row", 1000000 + num21);
            }
            x6 += num20;
          }
          rectangle5 = Rectangle::new(x6, num16, rectangle3.Width - x6, height2);
          trect = rectangle5;
          self.AddMouse( trect, "", "Click to select this row", 1000000 + num21);
          num16 += height2;
        }
      }
      let mut x7: i32 = rectangle4.X + 20;
      let mut y6: i32 = rectangle4.Y + 10;
      let mut se1LeaderSelected: i32 = self.game.EditObj.se1_leaderSelected;
      if (se1LeaderSelected > 0)
      {
        bitmap: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(se1LeaderSelected, 100, 140, true);
         let mut local3: &Graphics = &g;
         let mut local4: &Bitmap = &bitmap;
        rectangle5 = Rectangle::new(0, 0, bitmap.Width, bitmap.Height);
        let mut srcrect: &Rectangle = &rectangle5
        trect = Rectangle::new(x7, y6, 100, 140);
        let mut destrect: &Rectangle = &trect
        DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
        bitmap.Dispose();
        bitmap = (Bitmap) null;
        rectangle5 = Rectangle::new(x7, y6, 100, 140);
        trect = rectangle5;
        self.AddMouse( trect, "", "Click to get more information on this Leader", 20000000 + se1LeaderSelected);
        let mut num23: i32 = x7 + 120;
        tstring: String = self.game.Data.StringListObj[self.slotCharacter].GetData(0, se1LeaderSelected, 3) + " " + self.game.Data.StringListObj[self.slotCharacter].GetData(0, se1LeaderSelected, 4);
        characterJobTitle: String = self.game.EventRelatedObj.Helper_GetCharacterJobTitle(se1LeaderSelected);
        DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont3, num23, y6, Color.White);
        DrawMod.DrawTextColouredMarc( g, characterJobTitle, self.game.MarcFont4, num23, y6 + 20, Color.LightGray);
        let mut num24: i32 = y6 + 50;
        let mut num25: i32 = 150;
        let mut num26: i32 = 40;
        tsubpart3 =  new TextButtonPartClass("Call", num25, "Give this Leader a Vidcom Call.",  self.OwnBitmap, num23, num24, theight: num26, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.but1id = self.AddSubPart( tsubpart3, num23, num24, num25, num26, 1);
        let mut num27: i32 = num24 + 50;
        tsubpart3 =  new TextButtonPartClass("Stratagem", num25, "Play a Stratagem on this Leader.",  self.OwnBitmap, num23, num27, theight: num26, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.but2id = self.AddSubPart( tsubpart3, num23, num27, num25, num26, 1);
      }
      g.Dispose();
      g = (Graphics) null;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index1: i32 = 0; index1 <= mouseCounter; index1 += 1)
      {
        if (self.MouseData[index1] > 0 && x > self.MouseRect[index1].X & x < self.MouseRect[index1].X + self.MouseRect[index1].Width && y > self.MouseRect[index1].Y & y < self.MouseRect[index1].Y + self.MouseRect[index1].Height & b == 1)
        {
          if (self.MouseData[index1] > 50000 & self.MouseData[index1] < 60000)
          {
            let mut nr: i32 = self.game.EditObj.se1_leaderColumns.FindNr(self.tempColId[self.MouseData[index1] - 50000]);
            if (nr > -1)
            {
              int[] weight = self.game.EditObj.se1_leaderColumns.Weight;
              int[] numArray = weight;
              let mut index2: i32 = nr;
              let mut index3: i32 = index2;
              let mut num: i32 = weight[index2] + 1;
              numArray[index3] = num;
              if (self.game.EditObj.se1_leaderColumns.Weight[nr] > 2)
                self.game.EditObj.se1_leaderColumns.Weight[nr] = 0;
              let mut counter: i32 = self.game.EditObj.se1_leaderColumns.Counter;
              for (let mut index4: i32 = 0; index4 <= counter; index4 += 1)
              {
                if (index4 != nr)
                  self.game.EditObj.se1_leaderColumns.Weight[index4] = 0;
              }
            }
            self.ReCalculate();
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 1000000 & self.MouseData[index1] < 2000000)
          {
            if (self.game.EditObj.se1_leaderSelected == self.MouseData[index1] - 1000000)
              self.game.EditObj.se1_leaderSelected = -1;
            else
              self.game.EditObj.se1_leaderSelected = self.MouseData[index1] - 1000000;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 20000000 & self.MouseData[index1] < 21000000)
          {
            if (self.game.EditObj.se1_leaderSelected != self.MouseData[index1] - 20000000)
              self.game.EditObj.se1_leaderSelected = self.MouseData[index1] - 20000000;
            self.game.EditObj.UDSpopupText = "";
            self.formref.Cursor = Cursors.WaitCursor;
            self.game.EditObj.UDSClearInput();
            self.game.EventRelatedObj.SetUDSKey("CHARID", self.game.EditObj.se1_leaderSelected);
            self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 531, 0, 0));
            self.formref.Cursor = Cursors.Default;
            self.game.EditObj.PopupValue = 21;
            windowReturnClass1.AddCommand(5, 14);
            self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index5: i32 = 0; index5 <= subPartCounter; index5 += 1)
        {
          if (x > self.SubPartX[index5] & x < self.SubPartX[index5] + self.SubPartW[index5] && y > self.SubPartY[index5] & y < self.SubPartY[index5] + self.SubPartH[index5])
          {
            let mut num1: i32 = self.SubPartID[index5];
            if (num1 == self.but0id)
            {
              self.game.EditObj.se1_leaderColumns = SimpleList::new();
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_KEY_RELATION, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_ADVISOR, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_DIRECTOR, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_GOVERNOR, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_OHQ, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_RESERVE, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_SECRETARY, 0);
              self.game.EditObj.se1_leaderColumns.Add(self.FIX_JOB_SHQ, 0);
              self.ReCalculate();
              self.dostuff();
              windowReturnClass1.SetFlag(true);
              windowReturnClass1.AddCommand(4, 67);
              return windowReturnClass1;
            }
            if (num1 == self.but1id)
            {
              self.game.EditObj.UDSpopupText = "";
              self.formref.Cursor = Cursors.WaitCursor;
              self.game.EditObj.UDSClearInput();
              self.game.EventRelatedObj.SetUDSKey("CHOICE", self.game.EditObj.se1_leaderSelected);
              self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 546, 0, 0));
              self.formref.Cursor = Cursors.Default;
              self.game.EditObj.PopupValue = 21;
              windowReturnClass1.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.but2id)
            {
              self.game.EditObj.se1_CardsCategory = 8;
              self.game.EditObj.se1_CardsTarget = self.game.EditObj.se1_leaderSelected;
              self.game.EditObj.se1_CardsCard = -1;
              self.game.EditObj.se1_CardsPage = 1;
              self.game.EditObj.SetViewMode2 = 5;
              self.game.EditObj.se1_ManagementMode = false;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.OrderType = 0;
              if (self.game.ScreenHeight < 920)
              {
                self.game.EditObj.GuiDown = true;
                windowReturnClass1.AddCommand(3, 11);
              }
              else
                windowReturnClass1.AddCommand(3, 11);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.listId)
            {
              let mut num2: i32 = self.SubPartList[index5].Click(x - self.SubPartX[index5], y - self.SubPartY[index5]);
              if (num2 > -1)
              {
                self.SubPartFlag[index5] = true;
                self.game.EditObj.se1_leaderGroup = num2;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
              self.SubPartFlag[index5] = true;
              self.dostuff();
              windowReturnClass1.SetFlag(true);
              windowReturnClass1.AddCommand(4, 67);
              return windowReturnClass1;
            }
            if (num1 == self.list2Id)
            {
              let mut tid: i32 = self.SubPartList[index5].Click(x - self.SubPartX[index5], y - self.SubPartY[index5]);
              if (tid > 0)
              {
                self.SubPartFlag[index5] = true;
                let mut nr: i32 = self.game.EditObj.se1_leaderColumns.FindNr(tid);
                if (nr > -1)
                  self.game.EditObj.se1_leaderColumns.RemoveNr(nr);
                else if (self.curSelection < self.maxSelection)
                {
                  this += 1.curSelection;
                  self.game.EditObj.se1_leaderColumns.Add(tid, 0);
                }
                self.ReCalculate();
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
              switch (tid)
              {
                case -2:
                  self.SubPartFlag[index5] = true;
                  for (let mut counter: i32 = self.game.EditObj.se1_leaderColumns.Counter; counter >= 0; counter += -1)
                  {
                    if (self.ColList.FindData(self.game.EditObj.se1_leaderColumns.Id[counter], 1) == self.game.EditObj.se1_leaderGroup)
                      self.game.EditObj.se1_leaderColumns.RemoveNr(counter);
                  }
                  self.ReCalculate();
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                case 0:
                  self.SubPartFlag[index5] = true;
                  let mut counter1: i32 = self.ColList.Counter;
                  for (let mut index6: i32 = 0; index6 <= counter1; index6 += 1)
                  {
                    if (self.ColList.Data1[index6] == self.game.EditObj.se1_leaderGroup && self.game.EditObj.se1_leaderColumns.FindNr(self.ColList.Id[index6]) == -1 && self.curSelection < self.maxSelection)
                    {
                      this += 1.curSelection;
                      self.game.EditObj.se1_leaderColumns.Add(self.ColList.Id[index6], 0);
                    }
                  }
                  self.ReCalculate();
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                default:
                  self.SubPartFlag[index5] = true;
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
              }
            }
            else
            {
              let mut assetButtonCounter: i32 = self.assetButtonCounter;
              for (let mut index7: i32 = 0; index7 <= assetButtonCounter; index7 += 1)
              {
                if (self.assetButton[index7] == self.SubPartID[index5] && self.assetButtonData[index7] >= 51 & self.assetButtonData[index7] < 99)
                {
                  self.game.EditObj.se1_leaderPage = self.assetButtonData[index7] - 50;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
              }
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn PopUpRefresh() => self.dostuff();
  }
}
