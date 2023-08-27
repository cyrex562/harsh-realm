// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabCardsWindowClass2
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
  pub class TabCardsWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     w: i32;
     h: i32;
     detailnr: i32;
     hovernr: i32;
     lastActualCard: i32;
     currentCat: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     pageId: i32;
     miniId: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;
     categoryCount: i32;
     categoryName: Vec<String>;
     int[] categorySelectMode;
     maxMiniSelectValue: i32;
     miniSelectX: i32;
     miniSelectY: i32;
     miniSelectValue: i32;
     miniUnitSelect: i32;
     miniSelectLeader: i32;
     miniCatSelectValue: i32;
     bool[,] cardPlayable;
     string[,] cardWhyNot;
     bool donePrepareCardPlayable;
     AIMatrix zones;
     zoneName: Vec<String>;
     bool[] zoneVisible;
     int[] zoneRegimeId;
     mouseOverWhichTab: i32;
     viewMode: i32;
     rememberExtraS: String;
     pageNr: i32;
     nextId: i32;
     prevId: i32;
     next2Id: i32;
     prev2Id: i32;
     sizeId: i32;
     scrapId: i32;
     buyScrapId: i32;
     buyScrapId2: i32;
     bool first;
     bool scrapMode;
     scrapMouseOver: i32;
     cardSize: i32;

    pub TabCardsWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.categoryName = new string[20];
      self.categorySelectMode = new int[20];
      self.cardPlayable = new bool[1, 1];
      self.cardWhyNot = new string[1, 1];
      self.zoneName = new string[1];
      self.zoneVisible = new bool[1];
      self.zoneRegimeId = new int[1];
      self.scrapMode = false;
      self.scrapMouseOver = 0;
      self.w = trect.Width;
      self.h = trect.Height;
      self.detailnr = -1;
      self.lastActualCard = -1;
      self.hovernr = -1;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.viewMode = 1;
      self.cardSize = 2;
      if (self.w < 1200)
        self.cardSize = 1;
      self.first = true;
      self.game.DC2AIObj.SetTempHexNeighbours();
      self.zones = new AIMatrix( tGame.DC2AIObj);
      let mut mapWidth: i32 = tGame.Data.MapObj[0].MapWidth;
      let mut mapHeight: i32 = tGame.Data.MapObj[0].MapHeight;
      data: DataClass = tGame.Data;
      str: String = "Zones";
       local: String =  str;
      let mut libVar: i32 = data.FindLibVar( local, "SE_Data");
      let mut num1: i32 = mapWidth;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = mapHeight;
        for (let mut index2: i32 = 0; index2 <= num2; index2 += 1)
          self.zones.Value[index1, index2] = tGame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar);
      }
      self.currentCat = 1;
      self.categoryCount = 9;
      self.categoryName[1] = "Majors";
      self.categorySelectMode[1] = 1;
      self.categoryName[2] = "Minors";
      self.categorySelectMode[2] = 1;
      self.categoryName[3] = "Covert Ops";
      self.categorySelectMode[3] = 2;
      self.categoryName[4] = "Nation";
      self.categorySelectMode[4] = 0;
      self.categoryName[5] = "Tariffs";
      self.categorySelectMode[5] = 1;
      self.categoryName[6] = "HQs";
      self.categorySelectMode[6] = 3;
      self.categoryName[7] = "Zones";
      self.categorySelectMode[7] = 2;
      self.categoryName[8] = "Leaders";
      self.categorySelectMode[8] = 4;
      self.categoryName[9] = "Units";
      self.categorySelectMode[9] = 5;
      self.miniSelectValue = -1;
      self.miniSelectLeader = -1;
      self.game.EditObj.MiniMap = new Bitmap(10, 10);
      self.miniSelectX = self.game.SelectX;
      self.miniSelectY = self.game.SelectY;
      self.miniUnitSelect = self.game.EditObj.UnitSelected;
      self.miniCatSelectValue = -1;
      if (self.game.EditObj.se1_CardsCategory > 0)
      {
        self.currentCat = self.game.EditObj.se1_CardsCategory;
        self.pageNr = self.game.EditObj.se1_CardsPage;
        if (self.categorySelectMode[self.currentCat] == 4)
        {
          self.miniSelectLeader = self.game.EditObj.se1_CardsTarget;
          self.miniSelectValue =  Math.Round(Conversion.Val( self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0))].FindRow(0, self.miniSelectLeader)));
          if (self.miniSelectValue < 0)
            self.miniSelectLeader = -1;
        }
        if (self.categorySelectMode[self.currentCat] == 3)
        {
          self.miniUnitSelect = self.game.EditObj.se1_CardsTarget;
          if (self.miniUnitSelect > self.game.Data.UnitCounter | self.miniUnitSelect < 1)
            self.miniUnitSelect = -1;
          else if (self.game.Data.UnitObj[self.miniUnitSelect].Historical == -1)
            self.miniUnitSelect = -1;
        }
        if (self.categorySelectMode[self.currentCat] == 5)
        {
          self.miniUnitSelect = self.game.EditObj.se1_CardsTarget;
          if (self.miniUnitSelect > self.game.Data.UnitCounter | self.miniUnitSelect < 1)
            self.miniUnitSelect = -1;
          else if (self.game.Data.UnitObj[self.miniUnitSelect].Historical == -1)
            self.miniUnitSelect = -1;
        }
        if (self.categorySelectMode[self.currentCat] == 1)
        {
          self.miniSelectValue = self.game.EditObj.se1_CardsTarget;
          if (self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0))].FindRow(0, self.miniSelectValue) < 0)
            self.miniSelectValue = -1;
          bool flag = false;
          if (self.game.EditObj.se1_CardsSelectX > -1 & self.game.EditObj.se1_CardsSelectY > -1 & self.game.EditObj.se1_CardsSelectX <= self.game.Data.MapObj[0].MapWidth & self.game.EditObj.se1_CardsSelectY <= self.game.Data.MapObj[0].MapHeight)
          {
            if (self.game.Data.MapObj[0].HexObj[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY].Regime > -1)
            {
              if (self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY].Regime].id != self.miniSelectValue)
                flag = true;
            }
            else
              flag = true;
          }
          if (self.miniSelectValue > 0 & flag)
          {
            let mut num3: i32 = -1;
            let mut num4: i32 = -1;
            let mut num5: i32 = 0;
            do
            {
              let mut num6: i32 = mapWidth;
              for (let mut index3: i32 = 0; index3 <= num6; index3 += 1)
              {
                let mut num7: i32 = mapHeight;
                for (let mut index4: i32 = 0; index4 <= num7; index4 += 1)
                {
                  if (self.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1 && num5 == 1 | self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index3, index4].Regime].id == self.miniSelectValue & num3 == -1)
                  {
                    num3 = index3;
                    num4 = index4;
                  }
                }
              }
              num5 += 1;
            }
            while (num5 <= 1);
            if (num3 > -1)
            {
              self.game.EditObj.se1_CardsSelectX = num3;
              self.game.EditObj.se1_CardsSelectY = num4;
            }
            else
              self.miniSelectValue = -1;
          }
        }
        if (self.categorySelectMode[self.currentCat] == 2)
        {
          self.miniSelectValue = self.game.EditObj.se1_CardsTarget;
          if (self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0))].FindRow(0, self.miniSelectValue) < 0)
            self.miniSelectValue = -1;
          bool flag = false;
          if (self.game.EditObj.se1_CardsSelectX > -1 & self.game.EditObj.se1_CardsSelectY > -1 && self.zones.Value[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY] != self.miniSelectValue)
            flag = true;
          if (self.miniSelectValue > 0 & flag)
          {
            let mut num8: i32 = -1;
            let mut num9: i32 = -1;
            let mut num10: i32 = 0;
            do
            {
              let mut num11: i32 = mapWidth;
              for (let mut index5: i32 = 0; index5 <= num11; index5 += 1)
              {
                let mut num12: i32 = mapHeight;
                for (let mut index6: i32 = 0; index6 <= num12; index6 += 1)
                {
                  if (num10 == 1 | self.game.Data.MapObj[0].HexObj[index5, index6].MaxRecon > 0 && self.zones.Value[index5, index6] == self.miniSelectValue & num8 == -1)
                  {
                    num8 = index5;
                    num9 = index6;
                  }
                }
              }
              num10 += 1;
            }
            while (num10 <= 1);
            if (num8 > -1)
            {
              self.game.EditObj.se1_CardsSelectX = num8;
              self.game.EditObj.se1_CardsSelectY = num9;
            }
            else
              self.miniSelectValue = -1;
          }
        }
        if (self.game.EditObj.se1_CardsCard > -1)
        {
          let mut actionCardCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter;
          for (let mut index: i32 = 0; index <= actionCardCounter; index += 1)
          {
            if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index]].TempVar0 == self.game.EditObj.se1_CardsCard)
            {
              self.detailnr = index;
              break;
            }
          }
        }
        if (self.game.EditObj.se1_CardsSmallCards > 0)
          self.cardSize = self.game.EditObj.se1_CardsSmallCards;
        if (self.game.EditObj.se1_CardsViewMode > 0)
          self.viewMode = self.game.EditObj.se1_CardsViewMode;
        self.miniSelectX = self.game.EditObj.se1_CardsSelectX;
        self.miniSelectY = self.game.EditObj.se1_CardsSelectY;
      }
      if (self.miniSelectX < 0 | self.miniSelectY < 0)
      {
        self.miniSelectX = self.game.SelectX;
        self.miniSelectY = self.game.SelectY;
      }
      self.prepareTempValue4();
      self.donePrepareCardPlayable = false;
      self.dostuff();
    }

    pub fn CopyToEditObj()
    {
      self.game.EditObj.se1_CardsCategory = self.currentCat;
      if (self.categorySelectMode[self.currentCat] == 4)
        self.game.EditObj.se1_CardsTarget = self.miniSelectLeader;
      else if (self.categorySelectMode[self.currentCat] == 3)
        self.game.EditObj.se1_CardsTarget = self.miniUnitSelect;
      else if (self.categorySelectMode[self.currentCat] == 5)
        self.game.EditObj.se1_CardsTarget = self.miniUnitSelect;
      else if (self.categorySelectMode[self.currentCat] == 1)
        self.game.EditObj.se1_CardsTarget = self.miniSelectValue;
      else if (self.categorySelectMode[self.currentCat] == 2)
        self.game.EditObj.se1_CardsTarget = self.miniSelectValue;
      self.game.EditObj.se1_CardsPage = self.pageNr;
      if (self.detailnr > -1)
        self.game.EditObj.se1_CardsCard = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].TempVar0;
      else
        self.game.EditObj.se1_CardsCard = -1;
      self.game.EditObj.se1_CardsSmallCards = self.cardSize;
      self.game.EditObj.se1_CardsViewMode = self.viewMode;
      self.game.EditObj.se1_CardsSelectX = self.miniSelectX;
      self.game.EditObj.se1_CardsSelectY = self.miniSelectY;
    }

    pub fn prepareTempValue4()
    {
      self.game.HandyFunctionsObj.RedimTempValue4(-1);
      self.game.HandyFunctionsObj.RedimTempValue3(-1);
      self.game.EditObj.TempAI = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.game.EditObj.TempAI2 = new int[self.game.Data.MapObj[0].MapWidth + 1, self.game.Data.MapObj[0].MapHeight + 1];
      self.maxMiniSelectValue = -1;
      bool[] flagArray = new bool[self.game.Data.RegimeCounter + 1];
      if (self.categorySelectMode[self.currentCat] == 1)
      {
        let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            let mut regime: i32 = self.game.Data.MapObj[0].HexObj[index1, index2].Regime;
            if (regime > -1)
            {
              flagArray[regime] = true;
              self.game.EditObj.TempValue4[0].Value[index1, index2] = self.game.Data.RegimeObj[regime].id;
              if (index1 == self.miniSelectX & index2 == self.miniSelectY)
                self.miniSelectValue = self.game.Data.RegimeObj[regime].id;
            }
          }
        }
      }
      AIMatrix specialMask = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
      if (self.categorySelectMode[self.currentCat] == 2)
      {
        let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 = 0; index3 <= mapWidth1; index3 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
          {
            let mut num: i32 = self.zones.Value[index3, index4];
            if (num > 0 & (self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 | !self.game.Data.ShrowdOn))
            {
              self.game.EditObj.TempValue4[0].Value[index3, index4] = num;
              if (index3 == self.miniSelectX & index4 == self.miniSelectY)
                self.miniSelectValue = num;
              if (num > self.maxMiniSelectValue)
                self.maxMiniSelectValue = num;
            }
            specialMask.Value[index3, index4] = 0;
            if (self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 | !self.game.Data.ShrowdOn)
            {
              if ((self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 | !self.game.Data.ShrowdOn) & self.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1)
                specialMask.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].Regime + 2;
              else if (self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) > 0)
                specialMask.Value[index3, index4] = self.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(self.game.Data.Turn) + 2;
            }
          }
        }
        if (self.game.Data.UnitCounter > self.maxMiniSelectValue)
          self.maxMiniSelectValue = self.game.Data.UnitCounter;
        if (self.maxMiniSelectValue > 0)
        {
          let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
          if (self.maxMiniSelectValue < self.game.Data.StringListObj[stringListById1].GetHighestValue(0))
            self.maxMiniSelectValue = self.game.Data.StringListObj[stringListById1].GetHighestValue(0);
          self.zoneName = new string[self.maxMiniSelectValue + 1];
          self.zoneRegimeId = new int[self.maxMiniSelectValue + 1];
          self.zoneVisible = new bool[self.maxMiniSelectValue + 1];
          let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 288, 0, 0));
          let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length;
          for (let mut index: i32 = 0; index <= length; index += 1)
          {
            let mut idValue2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index, 0]));
            let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index, 8]));
            str: String = self.game.Data.StringListObj[stringListById1].Data[index, 7];
            self.zoneName[idValue2] = str;
            self.zoneRegimeId[idValue2] = num;
            if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, idValue2, 2, "recon", 3))) > -1 | num == self.game.Data.RegimeObj[self.game.Data.Turn].id)
              self.zoneVisible[idValue2] = true;
          }
          let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index5: i32 = 0; index5 <= mapWidth2; index5 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
            {
              let mut index7: i32 = self.zones.Value[index5, index6];
              if (index7 > 0 & (self.game.Data.MapObj[0].HexObj[index5, index6].MaxRecon > 0 | !self.game.Data.ShrowdOn))
                self.zoneVisible[index7] = true;
            }
          }
        }
      }
      if (self.categorySelectMode[self.currentCat] == 1 | self.categorySelectMode[self.currentCat] == 2)
      {
        AIMatrix aiMatrix1 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
        AIMatrix aiMatrix2 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
        AIMatrix aiMatrix3 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
        AIMatrix aiMatrix4 = new AIMatrix( self.game.DC2AIObj, self.game.Data.MapObj[0].MapWidth, self.game.Data.MapObj[0].MapHeight, 0, 0);
        let mut mapWidth3: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index8: i32 = 0; index8 <= mapWidth3; index8 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index9: i32 = 0; index9 <= mapHeight; index9 += 1)
          {
            self.game.EditObj.TempAI[index8, index9] = 0;
            aiMatrix4.Value[index8, index9] = self.game.Data.MapObj[0].HexObj[index8, index9].get_LastLT(self.game.Data.Turn);
            if (aiMatrix4.Value[index8, index9] < 0)
              aiMatrix4.Value[index8, index9] = 0;
            if (self.game.Data.MapObj[0].HexObj[index8, index9].MaxRecon > 0 | !self.game.Data.ShrowdOn)
            {
              aiMatrix1.Value[index8, index9] =  byte.MaxValue;
              aiMatrix2.Value[index8, index9] = self.game.EditObj.TempValue4[0].Value[index8, index9];
              if (aiMatrix2.Value[index8, index9] <= 0)
                aiMatrix2.Value[index8, index9] = 999999;
              aiMatrix3.Value[index8, index9] = self.game.Data.MapObj[0].HexObj[index8, index9].Regime + 2;
              aiMatrix4.Value[index8, index9] = self.game.Data.MapObj[0].HexObj[index8, index9].LandscapeType + 1;
            }
            else if (self.game.Data.MapObj[0].HexObj[index8, index9].get_LastReg(self.game.Data.Turn) > -1 && flagArray[self.game.Data.MapObj[0].HexObj[index8, index9].get_LastReg(self.game.Data.Turn)])
            {
              aiMatrix1.Value[index8, index9] =  byte.MaxValue;
              aiMatrix2.Value[index8, index9] = self.game.EditObj.TempValue4[0].Value[index8, index9];
              if (aiMatrix2.Value[index8, index9] <= 0 & self.categorySelectMode[self.currentCat] == 1)
                aiMatrix2.Value[index8, index9] = 999999;
              aiMatrix3.Value[index8, index9] = self.game.Data.MapObj[0].HexObj[index8, index9].get_LastReg(self.game.Data.Turn) + 2;
              aiMatrix4.Value[index8, index9] = self.game.Data.MapObj[0].HexObj[index8, index9].get_LastLT(self.game.Data.Turn) + 1;
            }
          }
        }
        if (self.categorySelectMode[self.currentCat] == 2)
          specialMask.ExpandAllNonZeroValuesForAnyRegime(15);
        aiMatrix1.ExpandAndRemovePercentageForAnyRegime( byte.MaxValue, 0.8f, true);
        if (self.categorySelectMode[self.currentCat] == 2)
          aiMatrix2.ExpandAllNonZeroValuesForAnyRegime(15, specialMask);
        else
          aiMatrix2.ExpandAllNonZeroValuesForAnyRegime(15);
        aiMatrix3.ExpandAllNonZeroValuesForAnyRegime(15);
        aiMatrix4.ExpandAllNonZeroValuesForAnyRegime(15);
        let mut mapWidth4: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index10: i32 = 0; index10 <= mapWidth4; index10 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index11: i32 = 0; index11 <= mapHeight; index11 += 1)
          {
            if (self.game.Data.MapObj[0].HexObj[index10, index11].MaxRecon > 0 | !self.game.Data.ShrowdOn)
            {
              aiMatrix1.Value[index10, index11] =  byte.MaxValue;
              aiMatrix2.Value[index10, index11] = self.game.EditObj.TempValue4[0].Value[index10, index11];
              aiMatrix3.Value[index10, index11] = self.game.Data.MapObj[0].HexObj[index10, index11].Regime + 2;
              aiMatrix4.Value[index10, index11] = self.game.Data.MapObj[0].HexObj[index10, index11].LandscapeType + 1;
            }
            else if (self.game.Data.MapObj[0].HexObj[index10, index11].get_LastReg(self.game.Data.Turn) > 0)
            {
              aiMatrix1.Value[index10, index11] =  byte.MaxValue;
              if (self.categorySelectMode[self.currentCat] == 1)
                aiMatrix2.Value[index10, index11] = self.game.EditObj.TempValue4[0].Value[index10, index11];
              aiMatrix3.Value[index10, index11] = self.game.Data.MapObj[0].HexObj[index10, index11].get_LastReg(self.game.Data.Turn) + 2;
              aiMatrix4.Value[index10, index11] = self.game.Data.MapObj[0].HexObj[index10, index11].get_LastLT(self.game.Data.Turn) + 1;
            }
            else if (aiMatrix1.Value[index10, index11] < 20)
            {
              aiMatrix1.Value[index10, index11] = 0;
              aiMatrix2.Value[index10, index11] = 0;
              aiMatrix3.Value[index10, index11] = 0;
              aiMatrix4.Value[index10, index11] = 0;
            }
            if (aiMatrix2.Value[index10, index11] == 999999)
              aiMatrix2.Value[index10, index11] = 0;
            if (self.categorySelectMode[self.currentCat] == 1)
            {
              self.game.EditObj.TempValue4[0].Value[index10, index11] = 0;
              if (aiMatrix3.Value[index10, index11] > 1)
                self.game.EditObj.TempValue4[0].Value[index10, index11] = self.game.Data.RegimeObj[aiMatrix3.Value[index10, index11] - 2].id;
            }
            else
              self.game.EditObj.TempValue4[0].Value[index10, index11] = aiMatrix2.Value[index10, index11];
            self.game.EditObj.TempValue3[0].Value[index10, index11] = aiMatrix1.Value[index10, index11];
            self.game.EditObj.TempAI[index10, index11] = aiMatrix3.Value[index10, index11] - 2;
            self.game.EditObj.TempAI2[index10, index11] = aiMatrix4.Value[index10, index11] - 1;
          }
        }
      }
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
      {
        if (self.game.Data.RegimeObj[index].id > self.maxMiniSelectValue)
        {
          self.maxMiniSelectValue = self.game.Data.RegimeObj[index].id;
          self.zoneName = (string[]) Utils.CopyArray((Array) self.zoneName, (Array) new string[self.maxMiniSelectValue + 1]);
          self.zoneRegimeId = (int[]) Utils.CopyArray((Array) self.zoneRegimeId, (Array) new int[self.maxMiniSelectValue + 1]);
          self.zoneVisible = (bool[]) Utils.CopyArray((Array) self.zoneVisible, (Array) new bool[self.maxMiniSelectValue + 1]);
        }
      }
    }

    pub fn prepareCardPlayable(SimpleList SL)
    {
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 277, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 184, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 188, 0, 0));
      if (self.donePrepareCardPlayable)
        return;
      if (self.game.Data.UnitCounter > self.maxMiniSelectValue)
        self.maxMiniSelectValue = self.game.Data.UnitCounter;
      if (self.game.Data.StringListObj[stringListById1].Length + 10 > self.maxMiniSelectValue)
        self.maxMiniSelectValue = self.game.Data.StringListObj[stringListById1].Length + 10;
      self.cardPlayable = new bool[SL.Counter + 1, self.maxMiniSelectValue + 1];
      self.cardWhyNot = new string[SL.Counter + 1, self.maxMiniSelectValue + 1];
      let mut counter: i32 = SL.Counter;
      for (let mut index1: i32 = 0; index1 <= counter; index1 += 1)
      {
        let mut cardinhandnr: i32 = SL.Data1[index1];
        let mut index2: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[cardinhandnr];
        if (self.game.Data.ActionCardObj[index2].AreaSlot > -1)
        {
          index3: i32;
          if (Strings.InStr(self.game.Data.ActionCardObj[index2].Title, "Zoo") > 0)
            index3 = index3;
          self.game.EditObj.DoCardSlot = cardinhandnr;
          self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, cardinhandnr);
          if (self.categorySelectMode[self.currentCat] == 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index4: i32 = 0; index4 <= mapWidth; index4 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index5: i32 = 0; index5 <= mapHeight; index5 += 1)
              {
                index3 = self.game.Data.MapObj[0].HexObj[index4, index5].Regime;
                if (index3 > -1)
                {
                  let mut id: i32 = self.game.Data.RegimeObj[index3].id;
                  if (self.game.Data.MapObj[0].HexObj[index4, index5].AreaCode[0] > 0)
                    self.cardPlayable[index1, id] = true;
                  else
                    self.cardWhyNot[index1, id] = self.game.EditObj.TempString2[0].Value[index4, index5];
                }
              }
            }
          }
          if (self.categorySelectMode[self.currentCat] == 2)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
            for (let mut index6: i32 = 0; index6 <= mapWidth; index6 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
              for (let mut index7: i32 = 0; index7 <= mapHeight; index7 += 1)
              {
                index3 = self.zones.Value[index6, index7];
                if (index3 > -1)
                {
                  if (self.game.Data.MapObj[0].HexObj[index6, index7].AreaCode[0] > 0)
                    self.cardPlayable[index1, index3] = true;
                  else
                    self.cardWhyNot[index1, index3] = self.game.EditObj.TempString2[0].Value[index6, index7];
                }
              }
            }
          }
        }
        else if (self.game.Data.ActionCardObj[index2].UnitSelect)
        {
          self.game.EditObj.DoCardSlot = cardinhandnr;
          self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, cardinhandnr);
          if (self.categorySelectMode[self.currentCat] == 3)
          {
            let mut unitCounter: i32 = self.game.Data.UnitCounter;
            for (let mut index8: i32 = 0; index8 <= unitCounter; index8 += 1)
            {
              if (self.game.Data.UnitObj[index8].TempUnitSelectable)
                self.cardPlayable[index1, index8] = true;
              else
                self.cardWhyNot[index1, index8] = "These cards can only be played on HQ units";
            }
          }
          if (self.categorySelectMode[self.currentCat] == 5)
          {
            let mut unitCounter: i32 = self.game.Data.UnitCounter;
            for (let mut index9: i32 = 0; index9 <= unitCounter; index9 += 1)
            {
              if (self.game.Data.UnitObj[index9].TempUnitSelectable)
                self.cardPlayable[index1, index9] = true;
              else
                self.cardWhyNot[index1, index9] = "These cards can only be played on non-HQ units";
            }
          }
        }
        else if (self.categorySelectMode[self.currentCat] == 4)
        {
          self.game.EditObj.DoCardSlot = cardinhandnr;
          self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, cardinhandnr);
          let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length;
          for (let mut index10: i32 = 0; index10 <= length; index10 += 1)
          {
            if (self.game.EditObj.tempOtherTest[index10] == 1)
            {
              self.cardPlayable[index1, index10] = true;
            }
            else
            {
              if (self.game.Data.ActionCardObj[index2].TempVar0 == 522)
                index1 = index1;
              self.cardWhyNot[index1, index10] = self.game.EditObj.tempOtherTestText[index10];
              self.cardPlayable[index1, index10] = false;
            }
            if (self.categorySelectMode[self.currentCat] == 4)
            {
              let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 277, 0, 0));
              let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.ActionCardObj[index2].TempVar0, 5)));
              let mut jobSpecificId: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData2(1, idValue1, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
              let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].GetData(0, idValue1, 2)));
              if (jobSpecificId < 1)
              {
                let mut idValue2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.ActionCardObj[index2].TempVar0, 14)));
                jobSpecificId =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData2(1, idValue2, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
                num1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].GetData(0, idValue2, 2)));
              }
              let mut idValue2_1: i32 = -1;
              if (jobSpecificId > 0)
              {
                switch (num1)
                {
                  case 1:
                    idValue2_1 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 5, jobSpecificId, -1);
                    break;
                  case 2:
                    idValue2_1 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 9, jobSpecificId, -1);
                    break;
                }
              }
              if (idValue2_1 < 1)
                idValue2_1 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 8, 0, -1);
              if (idValue2_1 ==  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index10, 0])))
              {
                let mut num2: i32 = 0;
                if (self.game.Data.StringListObj[stringListById4].Width >= 19)
                  num2 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.ActionCardObj[index2].TempVar0, 19)));
                if (num2 < 1)
                {
                  self.cardWhyNot[index1, index10] = "A Leader cannot execute a Stratagem on self";
                  self.cardPlayable[index1, index10] = false;
                }
              }
              else
              {
                let mut num3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(6, 6, 7, idValue2_1, 0)));
                if (num3 > 0 && num3 ==  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index10, 0])))
                {
                  let mut num4: i32 = 0;
                  if (self.game.Data.StringListObj[stringListById4].Width >= 19)
                    num4 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, self.game.Data.ActionCardObj[index2].TempVar0, 19)));
                  if (num4 < 1)
                  {
                    self.cardWhyNot[index1, index10] = "A Leader cannot execute a Stratagem on its own Advisor";
                    self.cardPlayable[index1, index10] = false;
                  }
                }
              }
            }
          }
        }
        else
        {
          self.game.EditObj.DoCardSlot = cardinhandnr;
          self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, cardinhandnr);
          if (self.game.EditObj.tempOtherTest.GetUpperBound(0) > 0)
          {
            if (self.game.EditObj.tempOtherTest[1] == 1)
            {
              self.cardPlayable[index1, 1] = true;
            }
            else
            {
              self.cardWhyNot[index1, 1] = self.game.EditObj.tempOtherTestText[1];
              self.cardPlayable[index1, 1] = false;
            }
          }
          else
            self.cardPlayable[index1, 1] = true;
        }
      }
      self.donePrepareCardPlayable = true;
    }

    pub fn DoRefresh()
    {
      if (self.detailnr <= -1 || self.lastActualCard == -1)
        return;
      if (self.detailnr > self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter)
        self.detailnr = -1;
      else if (self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr] != self.lastActualCard)
      {
        self.detailnr = -1;
        if (self.game.EditObj.se1_CardsCard > -1)
        {
          let mut actionCardCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter;
          for (let mut index: i32 = 0; index <= actionCardCounter; index += 1)
          {
            if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index]].TempVar0 == self.game.EditObj.se1_CardsCard)
            {
              self.detailnr = index;
              break;
            }
          }
        }
        if (self.detailnr == -1)
          self.game.EditObj.se1_CardsCard = -1;
      }
      self.lastActualCard = -1;
      self.dostuff();
    }

    pub fn dostuff()
    {
      let mut num1: i32 = self.game.EventRelatedObj.CheckStringlistID("SE_IO", 158, 0, 0);
      self.game.HandyFunctionsObj.GetStringListByID(num1);
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 277, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 278, 0, 0));
      let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 279, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 169, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 168, 0, 0));
      let mut stringListById5: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 184, 0, 0));
      let mut stringListById6: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 188, 0, 0));
      let mut stringListById7: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
      let mut stringListById8: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 169, 0, 0));
      let mut stringListById9: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 168, 0, 0));
      let mut stringListById10: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 203, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 199, 0, 0));
      let mut stringListById11: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
      let mut stringListById12: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0));
      let mut stringListById13: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      let mut num2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0))].GetData(0, 58, 2)));
      if (self.Info1Id > 0)
      {
        self.RemoveSubPart(self.Info1Id);
        self.Info1Id = 0;
      }
      if (self.sizeId > 0)
      {
        self.RemoveSubPart(self.sizeId);
        self.sizeId = 0;
      }
      if (self.scrapId > 0)
      {
        self.RemoveSubPart(self.scrapId);
        self.scrapId = 0;
      }
      if (self.buyScrapId > 0)
      {
        self.RemoveSubPart(self.buyScrapId);
        self.buyScrapId = 0;
      }
      if (self.buyScrapId2 > 0)
      {
        self.RemoveSubPart(self.buyScrapId2);
        self.buyScrapId2 = 0;
      }
      if (self.nextId > 0)
      {
        self.RemoveSubPart(self.nextId);
        self.nextId = 0;
      }
      if (self.prevId > 0)
      {
        self.RemoveSubPart(self.prevId);
        self.prevId = 0;
      }
      if (self.next2Id > 0)
      {
        self.RemoveSubPart(self.next2Id);
        self.next2Id = 0;
      }
      if (self.prev2Id > 0)
      {
        self.RemoveSubPart(self.prev2Id);
        self.prev2Id = 0;
      }
      if (self.info2id > 0)
      {
        self.RemoveSubPart(self.info2id);
        self.info2id = 0;
      }
      if (self.pageId > 0)
      {
        self.RemoveSubPart(self.pageId);
        self.pageId = 0;
      }
      if (self.miniId > 0)
      {
        self.RemoveSubPart(self.miniId);
        self.miniId = 0;
      }
      self.tSelectX = self.game.SelectX;
      self.tSelectY = self.game.SelectY;
      self.tCornerX = self.game.CornerX;
      self.tCornerY = self.game.CornerY;
      self.game.EditObj.AreaX = -1;
      self.game.EditObj.AreaY = -1;
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, "STRAT.", 5);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F6]", 9999999);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      SimpleList simpleList1 = SimpleList::new();
      int[] numArray1 = new int[20];
      let mut actionCardCounter1: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter;
      for (let mut index1: i32 = 0; index1 <= actionCardCounter1; index1 += 1)
      {
        let mut num3: i32 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1]].ColorScheme * 1000 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1];
        if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1]].Category > 0 && self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1]].quickButton < 2 && self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1]].Category < 99)
        {
          int[] numArray2 = numArray1;
          int[] numArray3 = numArray2;
          let mut category: i32 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index1]].Category;
          let mut index2: i32 = category;
          let mut num4: i32 = numArray2[category] + 1;
          numArray3[index2] = num4;
        }
      }
      if (self.first & self.currentCat > 0 && numArray1[self.currentCat] < 1)
      {
        self.currentCat = 0;
        self.miniSelectValue = -1;
      }
      if (self.currentCat < 1)
      {
        let mut categoryCount: i32 = self.categoryCount;
        for (let mut index: i32 = 1; index <= categoryCount; index += 1)
        {
          if (numArray1[index] > 0)
          {
            self.currentCat = index;
            self.prepareTempValue4();
            break;
          }
        }
      }
      if (self.currentCat < 1)
        self.currentCat = 1;
      if (numArray1[self.currentCat] < 1)
        self.viewMode = 1;
      self.first = false;
      self.game.EditObj.se1_CardsCategory = self.currentCat;
      let mut x1_1: i32 = 30;
      let mut y1_1: i32 = 42;
      let mut w1: i32 = 512;
      let mut h1: i32 = 512;
      if (self.w > 910)
        w1 += self.w - 910;
      if (self.h > 602)
        h1 += self.h - 602;
      let mut tx1: i32 = self.game.ScreenWidth >= 1400 ? x1_1 +  Math.Round( w1 / 2.0) - 200 : x1_1 +  Math.Round( w1 / 2.0) - 100;
      let mut ty1: i32 = -5;
      bool active1 = false;
      if (self.viewMode == 1)
        active1 = true;
      sText1: String = "STRATAGEMS";
      if (!active1)
        ty1 -= 6;
      trect1 = self.DrawOneTab(g, true, active1, tx1, ty1, "", sText1, -1, 4, tMousingOverNow: (self.mouseOverWhichTab == 1));
      self.AddMouse( trect1, "", "View the Stratagems available", 1);
      let mut tx2: i32 = tx1 + 200;
      bool active2 = false;
      let mut ty2: i32 = -5;
      if (self.viewMode == 2)
        active2 = true;
      sText2: String = "TARGET";
      if (!active2)
        ty2 -= 6;
      trect1 = self.DrawOneTab(g, true, active2, tx2, ty2, "", sText2, -1, 16, grayedOut: (numArray1[self.currentCat] == 0 | self.categorySelectMode[self.currentCat] == 0), tMousingOverNow: (self.mouseOverWhichTab == 2));
      if (!(numArray1[self.currentCat] == 0 | self.categorySelectMode[self.currentCat] == 0))
        self.AddMouse( trect1, "", "Select or change the Target of the Stratagem", 2);
      bool tselected = true;
      if (self.cardSize == 2)
        tselected = false;
      let mut num5: i32 = self.w - 200;
      let mut num6: i32 = 0;
      let mut tsubpart1: SubPartClass =  new MarcRadioPartClass(0, tselected, "Click to change size of the Stratagems",  self.BackBitmap, num5, num6);
      self.sizeId = self.AddSubPart( tsubpart1, num5, num6, 35, 35, 1);
      DrawMod.DrawTextColouredConsole( g, "Small Stratagems", self.game.MarcFont4, num5 + 40, num6 + 8, self.game.seColWhite);
      let mut num7: i32 = self.w - 340;
      let mut num8: i32 = 0;
      if ( Math.Round(Conversion.Val(self.game.Data.Designer)) >= 94)
      {
        let mut tsubpart2: SubPartClass =  new MarcRadioPartClass(0, self.scrapMode, "Click to switch on/off Scrap Mode. This mode allows you to Scrap Stratagems to earn Scrap Points. ",  self.BackBitmap, num7, num8);
        self.scrapId = self.AddSubPart( tsubpart2, num7, num8, 35, 35, 1);
        DrawMod.DrawTextColouredConsole( g, "Scrap Mode", self.game.MarcFont4, num7 + 40, num8 + 8, self.game.seColWhite);
      }
      Rectangle trect2;
      Rectangle trect3;
      if (self.viewMode == 1)
      {
        DrawMod.DrawBlockGradient2( g, x1_1, y1_1, w1, h1, Color.FromArgb(50, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
        DrawMod.DrawSimpleFrame( self.game.LINESFRAME,  g, x1_1 - 3, y1_1 - 3, w1 + 6, h1 + 6, 10, 10, 10);
        let mut x1_2: i32 = self.w - 335;
        let mut y1_2: i32 = 42;
        let mut w2: i32 = 300;
        let mut h2: i32 = 512;
        if (self.h > 602)
          h2 += self.h - 602;
        DrawMod.DrawBlockGradient2( g, x1_2, y1_2, w2, h2, Color.FromArgb(50, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
        DrawMod.DrawSimpleFrame( self.game.LINESFRAME,  g, x1_2 - 3, y1_2 - 3, w2 + 6, h2 + 6, 10, 10, 10);
        DrawMod.drawLineDot( g, 176, 43, 176, self.h - 80, Color.White);
        let mut categoryCount: i32 = self.categoryCount;
        for (let mut index: i32 = 1; index <= categoryCount; index += 1)
        {
          let mut num9: i32 = numArray1[index];
          if (num9 > 6)
            num9 = 6;
          if (num9 > 0 & self.currentCat < 1)
            self.currentCat = index;
          if (index == self.currentCat)
            DrawMod.DrawBlockGradient( g, 36, 43 + 50 * (index - 1), 140, 50, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
          trect2 = Rectangle::new(36, 43 + 50 * (index - 1), 140, 50);
          trect3 = trect2;
          self.AddMouse( trect3, "Stratagem Category " + self.categoryName[index], "Click to see all stratagems in this category", 100000 + index);
          let mut num10: i32 = 50 * index;
          color: Color = self.game.seColWhite;
          if (index != self.currentCat)
            color = self.game.seColGray;
          if (numArray1[index] < 1)
          {
            c: Color = DrawMod.LightenColor(color, -50);
            DrawMod.DrawTextColouredMarcCenter( g, self.categoryName[index], self.game.MarcFont4, 106, num10 + 10, c);
          }
          else
          {
            DrawMod.DrawTextColouredMarcCenter( g, self.categoryName[index], self.game.MarcFont4, 106, num10 - 1, color);
            DrawMod.DrawTextColouredMarcCenter( g, "( " + numArray1[index].ToString() + " )", self.game.MarcFont5, 106, num10 + 24, color);
          }
          DrawMod.drawLineDot( g, 56, 43 + 50 * index, 176, 43 + 50 * index, Color.White);
        }
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      Rectangle rectangle3;
      if (self.viewMode == 2 & (self.categorySelectMode[self.currentCat] == 1 | self.categorySelectMode[self.currentCat] == 2))
      {
        let mut x1_3: i32 = self.w - 335;
        let mut y1_3: i32 = 42;
        let mut w3: i32 = 300;
        let mut h3: i32 = 512;
        if (self.h > 602)
          h3 += self.h - 602;
        DrawMod.DrawBlockGradient2( g, x1_3, y1_3, w3, h3, Color.FromArgb(50, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
        DrawMod.DrawSimpleFrame( self.game.LINESFRAME,  g, x1_3 - 3, y1_3 - 3, w3 + 6, h3 + 6, 10, 10, 10);
        let mut num11: i32 = 221;
        let mut num12: i32 = 42;
        let mut num13: i32 = 321;
        let mut num14: i32 = 240;
        if (self.w > 910)
          num13 += self.w - 910;
        if (self.h > 602)
          num14 += self.h - 602;
        rectangle1 = Rectangle::new(num11, num12, num13, num14);
        DrawMod.DrawBlockGradient2( g, num11, num12, num13, num14, Color.FromArgb(50, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
        DrawMod.DrawSimpleFrame( self.game.LINESFRAME,  g, num11 - 3, num12 - 3, num13 + 6, num14 + 6, 10, 10, 10);
        let mut y1: i32 = num12 + num14 + 30;
        let mut height1: i32 = 240;
        rectangle2 = Rectangle::new(num11, y1, num13, height1);
        let mut x: i32 = 30;
        let mut y2: i32 = 38;
        let mut width: i32 = 155;
        let mut height2: i32 = 512;
        if (self.h > 602)
          height2 += self.h - 602;
        rectangle3 = Rectangle::new(x, y2, width, height2);
      }
      Rectangle rectangle4;
      if (self.viewMode == 2 & (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5 | self.categorySelectMode[self.currentCat] == 4))
      {
        let mut x1_4: i32 = self.w - 335;
        let mut y1_4: i32 = 42;
        let mut w4: i32 = 300;
        let mut h4: i32 = 512;
        if (self.h > 602)
          h4 += self.h - 602;
        DrawMod.DrawBlockGradient2( g, x1_4, y1_4, w4, h4, Color.FromArgb(50, 0, 0, 0), Color.FromArgb(150, 0, 0, 0));
        DrawMod.DrawSimpleFrame( self.game.LINESFRAME,  g, x1_4 - 3, y1_4 - 3, w4 + 6, h4 + 6, 10, 10, 10);
        let mut x1: i32 = 221;
        let mut y3: i32 = 42;
        let mut width1: i32 = 321;
        let mut height3: i32 = 240;
        if (self.w > 910)
          width1 += self.w - 910;
        if (self.h > 602)
          height3 += self.h - 602;
        rectangle3 = Rectangle::new(x1, y3, width1, height3);
        let mut y4: i32 = y3 + height3 + 30;
        let mut height4: i32 = 240;
        rectangle2 = Rectangle::new(x1, y4, width1, height4);
        let mut x2: i32 = 30;
        let mut y5: i32 = 38;
        let mut width2: i32 = 155;
        let mut height5: i32 = 512;
        if (self.h > 602)
          height5 += self.h - 602;
        rectangle4 = Rectangle::new(x2, y5, width2, height5);
      }
      self.game.Data.StringListObj[stringListById8].SetData(0, "ZONEID", 1, 0);
      self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCHARID", 1, 0);
      let mut id1: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      self.game.Data.StringListObj[stringListById8].SetData(0, "REGIMEID", 1, id1, true);
      self.game.Data.StringListObj[stringListById8].SetData(0, "SOURCEAI", 1, 0, true);
      self.game.Data.StringListObj[stringListById8].SetData(0, "REGID", 1, id1, true);
      self.game.Data.StringListObj[stringListById8].SetData(0, "SOURCEREGIMEID", 1, id1, true);
      self.game.Data.StringListObj[stringListById8].SetData(0, "SOURCEREGIMESLOT", 1, self.game.Data.Turn, true);
      self.game.Data.StringListObj[stringListById8].SetData(0, "ROUND", 1, self.game.Data.Round, true);
      if (self.categorySelectMode[self.currentCat] == 1)
      {
        self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
        if (self.miniSelectValue > -1)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEID", 1, self.miniSelectValue, true);
          let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(self.miniSelectValue);
          if (regimeById == -1)
          {
            self.miniSelectValue = -1;
          }
          else
          {
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMESLOT", 1, regimeById, true);
            let mut num15: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 2)));
            let mut setValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById13].GetData(0, num15, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTURE", 1, setValue1, true);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTUREID", 1, num15, true);
            let mut setValue2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 13)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEAIID", 1, setValue2, true);
            let mut num16: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regimeById];
            let mut num17: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 1)));
            if (num16 == 0 & num17 == 2 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById12].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[regimeById].id, 2, "dipClear", 3))) < 1)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            else if (num16 == 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 1, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
          }
        }
      }
      else if (self.categorySelectMode[self.currentCat] == 2)
      {
        self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        self.game.Data.StringListObj[stringListById8].SetData(0, "ZONEID", 1, self.miniSelectValue, true);
        self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
        if (self.miniSelectValue > -1)
        {
          let mut num18: i32 = self.zoneRegimeId[self.miniSelectValue];
          self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEID", 1, num18, true);
          let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(num18);
          if (regimeById == -1)
          {
            self.miniSelectValue = -1;
          }
          else
          {
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMESLOT", 1, regimeById, true);
            let mut num19: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num18, 2)));
            let mut setValue3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById13].GetData(0, num19, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTURE", 1, setValue3, true);
            let mut setValue4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num18, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETMAJOR", 1, setValue4, true);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTUREID", 1, num19, true);
            let mut setValue5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num18, 13)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEAIID", 1, setValue5, true);
            let mut num20: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regimeById];
            let mut num21: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num18, 1)));
            if (num20 == 0 & num21 == 2 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById12].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[regimeById].id, 2, "dipClear", 3))) < 1)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            else if (num20 == 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 1, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            let mut setValue6: i32 = 0;
            if (self.game.Data.RegimeObj[self.game.Data.Turn].AI)
              setValue6 = 1;
            self.game.Data.StringListObj[stringListById8].SetData(0, "SOURCEAI", 1, setValue6, true);
            let mut setValue7: i32 = 0;
            if (self.game.Data.RegimeObj[regimeById].AI)
              setValue7 = 1;
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETAI", 1, setValue7, true);
          }
        }
      }
      else if (self.categorySelectMode[self.currentCat] == 3)
      {
        self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, self.miniUnitSelect, true);
        if (self.miniUnitSelect > 0)
        {
          if (self.game.Data.UnitObj[self.miniUnitSelect].Historical > 0)
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.miniUnitSelect].Historical].ID, true);
          else
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        }
        else
          self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
      }
      else if (self.categorySelectMode[self.currentCat] == 5)
      {
        self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, self.miniUnitSelect, true);
        if (self.miniUnitSelect > 0)
        {
          if (self.game.Data.UnitObj[self.miniUnitSelect].Historical > 0)
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.miniUnitSelect].Historical].ID, true);
          else
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        }
        else
          self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
      }
      else if (self.categorySelectMode[self.currentCat] == 4)
      {
        self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCHARID", 1, self.miniSelectLeader, true);
        self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
      }
      self.game.Data.StringListObj[stringListById8].SetData(0, "CHARID", 1, 0, true);
      SimpleList simpleList2 = SimpleList::new();
      let mut actionCardCounter2: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter;
      for (let mut tdata1: i32 = 0; tdata1 <= actionCardCounter2; tdata1 += 1)
      {
        if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1]].Category == self.currentCat | self.currentCat < 1 && self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1]].quickButton < 2)
        {
          let mut tweight: i32 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1]].ColorScheme * 100000 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1];
          simpleList2.Add(self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1]].TempVar0, tweight, tdata1, tdata5: self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[tdata1]].TempVar0, CheckExistence: false);
        }
      }
      simpleList2.Sort();
      SimpleList SL = SimpleList::new();
      let mut index3: i32 = -1;
      let mut num22: i32 = 0;
      let mut num23: i32 = -1;
      int[] numArray4 = new int[Math.Max(0, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter) + 1];
      let mut counter1: i32 = simpleList2.Counter;
      for (let mut index4: i32 = 0; index4 <= counter1; index4 += 1)
      {
        if (simpleList2.Id[index4] == index3)
        {
          num22 += 1;
        }
        else
        {
          index3 = simpleList2.Id[index4];
          num22 = 1;
        }
        if (index3 == num23)
        {
          SL.AddWeight(index3, 1, simpleList2.Data1[index4], tdata5: simpleList2.Data5[index4]);
        }
        else
        {
          num23 = index3;
          SL.AddWeight(index3, 1, simpleList2.Data1[index4], tdata5: simpleList2.Data5[index4]);
        }
        let mut nr: i32 = SL.FindNr(index3);
        numArray4[simpleList2.Data1[index4]] = nr;
      }
      StringListClass stringListClass = self.game.Data.StringListObj[stringListById8].Clone();
      if (self.game.Data.UnitCounter > self.cardPlayable.GetUpperBound(1) & (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5))
        self.donePrepareCardPlayable = false;
      if (self.categorySelectMode[self.currentCat] == 4 & self.miniSelectLeader > 0 && self.game.Data.StringListObj[stringListById7].FindRow(0, self.miniSelectLeader) == -1)
        self.donePrepareCardPlayable = false;
      if (!self.donePrepareCardPlayable)
        self.prepareTempValue4();
      self.prepareCardPlayable(SL);
      self.game.Data.StringListObj[stringListById8] = stringListClass;
      if (self.detailnr > self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter)
        self.detailnr = -1;
      if (self.hovernr > self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter)
        self.hovernr = -1;
      bitmap: Bitmap;
      if (self.viewMode == 1)
      {
        let mut num24: i32 = 186;
        let mut num25: i32 = 43;
        let mut num26: i32 = 359;
        let mut num27: i32 = 459;
        if (self.w > 910)
          num26 += self.w - 910;
        if (self.h > 602)
          num27 += self.h - 602;
        let mut num28: i32 = 115;
        let mut num29: i32 = 157;
        if (self.cardSize == 2)
        {
          num28 = 200;
          num29 = 276;
        }
        let mut num30: i32 =  Math.Round(Math.Floor( num26 /  num28));
        let mut num31: i32 =  Math.Round(Math.Floor( num27 /  num29));
        let mut num32: i32 = num30;
        let mut num33: i32 = num31;
        let mut num34: i32 = 1;
        num35: i32;
        num36: i32;
        num37: i32;
        num38: i32;
        do
        {
          num35 = num32;
          num36 = num33;
          if (self.pageNr < 1)
            self.pageNr = 1;
          num37 = num35 * num36;
          let mut d: i32 = SL.Counter + 1 - (self.pageNr - 1) * num37;
          if (d > num37)
            d = num37;
          if (d == 0)
          {
            num35 = 1;
            num36 = 1;
          }
          else
          {
            let mut num39: i32 =  Math.Round(Math.Ceiling(Math.Sqrt( d)));
            if (num39 < num35)
              num35 = num39;
            if (num39 < num36)
              num36 = num39;
            if (num35 * num36 < d)
            {
              num35 = num32;
              num36 = num33;
            }
            while (num35 * (num36 - 1) >= d)
              --num36;
            if (d <= num35)
              num36 = 1;
          }
          num38 =  Math.Round(Math.Ceiling( (SL.Counter + 1) /  num37));
          if (self.pageNr > num38)
          {
            self.pageNr = num38;
            num34 += 1;
          }
          else
            break;
        }
        while (num34 <= 2);
        tstring1: String = "Page " + self.pageNr.ToString() + " of " + num38.ToString();
        DrawMod.DrawTextColouredConsoleCenter( g, tstring1, self.game.MarcFont4, num24 +  Math.Round( num26 / 2.0), num25 + num27 + 3, self.game.seColWhite);
        if (self.pageNr > 1)
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("<", 50, "Previous Page",  self.OwnBitmap, num24 +  Math.Round( num26 / 2.0) - 150, num25 + num27, theight: 30, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.prevId = self.AddSubPart( tsubpart3, num24 +  Math.Round( num26 / 2.0) - 100, num25 + num27, 50, 30, 1);
        }
        else
        {
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("<", 50, "Previous Page",  self.OwnBitmap, num24 +  Math.Round( num26 / 2.0) - 150, num25 + num27, true, theight: 30, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.prev2Id = self.AddSubPart( tsubpart4, num24 +  Math.Round( num26 / 2.0) - 100, num25 + num27, 50, 30, 0);
        }
        if (self.pageNr < num38)
        {
          let mut tsubpart5: SubPartClass =  new TextButtonPartClass(">", 50, "Next Page",  self.OwnBitmap, num24 +  Math.Round( num26 / 2.0) + 60, num25 + num27, theight: 30, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.nextId = self.AddSubPart( tsubpart5, num24 +  Math.Round( num26 / 2.0) + 60, num25 + num27, 50, 30, 1);
        }
        else
        {
          let mut tsubpart6: SubPartClass =  new TextButtonPartClass(">", 50, "Next Page",  self.OwnBitmap, num24 +  Math.Round( num26 / 2.0) + 60, num25 + num27, true, theight: 30, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.next2Id = self.AddSubPart( tsubpart6, num24 +  Math.Round( num26 / 2.0) + 60, num25 + num27, 50, 30, 0);
        }
        index3 = num26 - num35 * num28;
        let mut num40: i32 = num27 - num36 * num29;
        let mut num41: i32 = 186 +  Math.Round( index3 / 2.0);
        let mut num42: i32 = 43 +  Math.Round( num40 / 2.0) + 10;
        let mut num43: i32 = num35 * num28;
        let mut num44: i32 = num36 * num29;
        let mut num45: i32 = num41;
        let mut y6: i32 = num42;
        let mut x3: i32 = num45 - num28;
        self.rememberExtraS = "";
        let mut counter2: i32 = SL.Counter;
        for (let mut index5: i32 = 0; index5 <= counter2; index5 += 1)
        {
          if (index5 >= (self.pageNr - 1) * num37 & index5 < self.pageNr * num37)
          {
            x3 += num28;
            if (x3 >= num41 + num43)
            {
              x3 = num41;
              y6 += num29;
            }
            if (y6 <= num42 + num44)
            {
              let mut regcardslot: i32 = SL.Data1[index5];
              let mut num46: i32 = SL.Weight[index5];
              index3 = -1;
              bool flag = false;
              if (self.detailnr == -1)
                self.detailnr = regcardslot;
              if (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5)
              {
                if (self.miniUnitSelect > 0 && self.cardPlayable[index5, self.miniUnitSelect])
                  flag = true;
              }
              else if (self.categorySelectMode[self.currentCat] == 4)
              {
                if (self.miniSelectValue > -1)
                {
                  let mut row: i32 = self.game.Data.StringListObj[stringListById7].FindRow(0, self.miniSelectValue);
                  if (row > -1 && self.cardPlayable[index5, row])
                    flag = true;
                }
              }
              else if (self.miniSelectX > -1 & self.miniSelectY > -1 & self.categorySelectMode[self.currentCat] > 0)
              {
                index3 = self.game.EditObj.TempValue4[0].Value[self.miniSelectX, self.miniSelectY];
                if (index3 > -1 & index3 < 9999)
                {
                  if (self.cardPlayable[index5, index3])
                    flag = true;
                }
                else
                  flag = false;
              }
              else
                flag = true;
              if (regcardslot == self.detailnr && !self.scrapMode)
              {
                if (self.cardSize == 2)
                {
                   let mut local1: &Graphics = &g;
                  bitmap = BitmapStore.GetBitmap(self.game.SECARDOUTLINE);
                   let mut local2: &Bitmap = &bitmap;
                  let mut x4: i32 = x3 - 10;
                  let mut y7: i32 = y6 - 10;
                  DrawMod.DrawScaled( local1,  local2, x4, y7, 210, 286);
                }
                else
                {
                   let mut local3: &Graphics = &g;
                  bitmap = BitmapStore.GetBitmap(self.game.SECARDOUTLINE);
                   let mut local4: &Bitmap = &bitmap;
                  let mut x5: i32 = x3 - 10;
                  let mut y8: i32 = y6 - 10;
                  DrawMod.DrawSimple( local3,  local4, x5, y8);
                }
              }
              if (self.cardSize == 2)
              {
                 let mut local5: &Graphics = &g;
                bitmap = self.game.CustomBitmapObj.DrawActionCardSe1(self.game.Data.Turn, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]);
                 let mut local6: &Bitmap = &bitmap;
                let mut x6: i32 = x3;
                let mut y9: i32 = y6;
                DrawMod.DrawSimple( local5,  local6, x6, y9);
              }
              else
              {
                 let mut local7: &Graphics = &g;
                bitmap = self.game.CustomBitmapObj.DrawActionCardSe1(self.game.Data.Turn, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot], size: 2);
                 let mut local8: &Bitmap = &bitmap;
                let mut x7: i32 = x3;
                let mut y10: i32 = y6;
                DrawMod.DrawSimple( local7,  local8, x7, y10);
              }
              let mut num47: i32 = SL.Weight[index5];
              if (num47 > 1)
              {
                 let mut local9: &Graphics = &g;
                bitmap = BitmapStore.GetBitmap(self.game.SE1_MULTICARD);
                 let mut local10: &Bitmap = &bitmap;
                let mut x8: i32 = x3 - 6;
                let mut y11: i32 = y6 + num29 - 42;
                DrawMod.DrawSimple( local9,  local10, x8, y11);
                tstring2: String = num47.ToString();
                if (num47 > 9)
                  tstring2 = "9+";
                DrawMod.DrawTextColouredConsoleCenter( g, tstring2, self.game.MarcFont3, x3 + 12, y6 + num29 - 32, self.game.seColWhite);
              }
              str1: String = "";
              let mut num48: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, SL.Id[index5], 13)));
              let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, SL.Id[index5], 5)));
              let mut idValue2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, SL.Id[index5], 14)));
              let mut num49: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, SL.Id[index5], 7)));
              let mut index6: i32 = stringListById3;
              if (num49 == 1)
              {
                index3 = self.game.HandyFunctionsObj.GetRegimeByID(self.game.EditObj.TempValue4[0].Value[self.miniSelectX, self.miniSelectY]);
                if (index3 > -1 && !self.game.Data.RegimeObj[index3].AI & num48 == 1)
                  index6 = stringListById4;
              }
              String2: String = "[" + SL.Id[index5].ToString() + "]";
              let mut length1: i32 = self.game.Data.StringListObj[index6].Length;
              for (let mut index7: i32 = 0; index7 <= length1; index7 += 1)
              {
                if (Strings.InStr(self.game.Data.StringListObj[index6].Data[index7, 0], String2) > 0)
                {
                  str2: String = self.game.Data.StringListObj[index6].Data[index7, 6];
                  if (str2.Length > 1)
                    str1 = str1 + "• " + str2 + "\r\n";
                }
              }
              if (str1.Length > 1)
              {
                let mut num50: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData2(1, idValue1, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
                let mut num51: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue1, 2)));
                if (num50 < 1)
                {
                  num50 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData2(1, idValue2, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
                  num51 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue2, 2)));
                }
                let mut num52: i32 = -1;
                if (num50 > 0)
                {
                  switch (num51)
                  {
                    case 1:
                      num52 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 5, num50, -1);
                      break;
                    case 2:
                      num52 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 9, num50, -1);
                      break;
                  }
                  if (self.categorySelectMode[self.currentCat] == 2 & num52 < 1)
                  {
                    let mut characterId: i32 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 10, self.miniSelectValue, -1);
                    if (characterId > 0)
                      num52 = characterId;
                  }
                }
                else if (self.categorySelectMode[self.currentCat] == 2)
                {
                  let mut characterId: i32 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 10, self.miniSelectValue, -1);
                  if (characterId > 0)
                    num52 = characterId;
                }
                if (num52 < 1)
                  num52 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 8, 0, -1);
                self.game.Data.StringListObj[stringListById8].SetData(0, "ORGID", 1, num50, true);
                if (num52 > 0)
                  self.game.Data.StringListObj[stringListById8].SetData(0, "CHARID", 1, num52, true);
                if (num52 > 0)
                {
                  setValue: String = self.game.EventRelatedObj.Helper_GetCharacterJobTitle(num52) + " " + self.game.Data.StringListObj[stringListById7].GetData(0, num52, 4);
                  self.game.Data.StringListObj[stringListById8].SetData(0, "CHARNAME", 1, setValue, true);
                }
                let mut length2: i32 = self.game.Data.StringListObj[stringListById8].Length;
                for (let mut index8: i32 = 0; index8 <= length2; index8 += 1)
                {
                  str3: String = self.game.Data.StringListObj[stringListById8].Data[index8, 0];
                  newValue: String = self.game.Data.StringListObj[stringListById8].Data[index8, 1];
                  if (Strings.InStr(str1, "<" + str3 + ">") > 0)
                    str1 = str1.Replace("<" + str3 + ">", newValue);
                }
                str1 = self.game.EventRelatedObj.Helper_Lookup(str1);
                if (str1.Length <= 1)
                  ;
              }
              if (self.hovernr == regcardslot)
                self.rememberExtraS = str1;
              else if (self.detailnr == regcardslot)
                self.rememberExtraS = str1;
              if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver.Length > 1)
              {
                if (str1.Length > 1)
                  str1 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver + "\r\n\r\n" + "Effects when played:".ToUpper() + "\r\n" + str1;
                else
                  str1 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver;
              }
              if (Information.IsNothing( self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver))
                self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver = "";
              if (self.scrapMode)
              {
                let mut tdata2: i32 = self.game.EventRelatedObj.CardScrapValue(regcardslot);
                if (self.cardSize == 2)
                {
                  DrawMod.DrawBlock( g, x3 + 42, y6 + 238, 130, 31, 0, 0, 0,  byte.MaxValue);
                  let mut num53: i32 = 64;
                  if (self.scrapMouseOver == regcardslot)
                  {
                     let mut local11: &Graphics = &g;
                    bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
                     let mut local12: &Bitmap = &bitmap;
                    trect3 = Rectangle::new(num53 * 42, 32, 42, 32);
                    let mut srcrect: &Rectangle = &trect3
                    trect2 = Rectangle::new(x3 + 40, y6 + 237, 42, 32);
                    let mut destrect: &Rectangle = &trect2
                    DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
                    DrawMod.DrawTextColouredConsole( g, tdata2.ToString() + " Scrap Pts", self.game.MarcFont4, x3 + 80, y6 + 244, Color.White);
                  }
                  else
                  {
                     let mut local13: &Graphics = &g;
                    bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
                     let mut local14: &Bitmap = &bitmap;
                    trect3 = Rectangle::new(num53 * 42, 0, 42, 32);
                    let mut srcrect: &Rectangle = &trect3
                    trect2 = Rectangle::new(x3 + 40, y6 + 237, 42, 32);
                    let mut destrect: &Rectangle = &trect2
                    DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
                    DrawMod.DrawTextColouredConsole( g, tdata2.ToString() + " Scrap Pts", self.game.MarcFont4, x3 + 80, y6 + 244, Color.LightGray);
                  }
                  trect3 = Rectangle::new(x3 + 42, y6 + 238, 130, 31);
                  trect2 = trect3;
                  self.AddMouse( trect2, "Scrap!", "Click to Scrap 1 Stratagem", regcardslot + 50100, tdata2);
                  if (self.scrapMouseOver == regcardslot)
                  {
                    DrawMod.DrawRectangle( g, x3 + 42, y6 + 238, 130, 31,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                    DrawMod.DrawRectangle( g, x3 + 42 - 1, y6 + 238 - 1, 132, 33,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
                    DrawMod.DrawRectangle( g, x3 + 42 + 1, y6 + 238 + 1, 128, 29,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
                  }
                }
                if (self.cardSize == 1)
                {
                  DrawMod.DrawBlock( g, x3 + 37 - 15, y6 + 119 - 13, 60, 32, 0, 0, 0,  byte.MaxValue);
                  let mut num54: i32 = 64;
                  if (self.scrapMouseOver == regcardslot)
                  {
                     let mut local15: &Graphics = &g;
                    bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
                     let mut local16: &Bitmap = &bitmap;
                    trect3 = Rectangle::new(num54 * 42, 32, 42, 32);
                    let mut srcrect: &Rectangle = &trect3
                    trect2 = Rectangle::new(x3 + 30 - 15, y6 + 119 - 13, 42, 32);
                    let mut destrect: &Rectangle = &trect2
                    DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
                    DrawMod.DrawTextColouredConsole( g, tdata2.ToString() + " Pts", self.game.MarcFont5, x3 + 64 - 15, y6 + 130 - 13, Color.White);
                  }
                  else
                  {
                     let mut local17: &Graphics = &g;
                    bitmap = BitmapStore.GetBitmap(self.game.SE1_ICONS);
                     let mut local18: &Bitmap = &bitmap;
                    trect3 = Rectangle::new(num54 * 42, 0, 42, 32);
                    let mut srcrect: &Rectangle = &trect3
                    trect2 = Rectangle::new(x3 + 30 - 15, y6 + 119 - 13, 42, 32);
                    let mut destrect: &Rectangle = &trect2
                    DrawMod.DrawSimplePart2( local17,  local18, srcrect, destrect);
                    DrawMod.DrawTextColouredConsole( g, tdata2.ToString() + " Pts", self.game.MarcFont5, x3 + 64 - 15, y6 + 130 - 13, Color.LightGray);
                  }
                  trect3 = Rectangle::new(x3 + 37 - 15, y6 + 119 - 13, 60, 32);
                  trect2 = trect3;
                  self.AddMouse( trect2, "Scrap!", "Click to Scrap 1 Stratagem", regcardslot + 50100, tdata2);
                  if (self.scrapMouseOver == regcardslot)
                  {
                    DrawMod.DrawRectangle( g, x3 + 37 - 15, y6 + 119 - 13, 60, 32,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                    DrawMod.DrawRectangle( g, x3 + 37 - 15 - 1, y6 + 119 - 13 - 1, 62, 34,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
                    DrawMod.DrawRectangle( g, x3 + 37 - 15 + 1, y6 + 119 - 13 + 1, 58, 30,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 128);
                  }
                }
              }
              if (!self.scrapMode)
              {
                if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].MouseOver.Length > 0)
                {
                  trect3 = Rectangle::new(x3, y6, num28 - 10, num29 - 10);
                  trect2 = trect3;
                  self.AddMouse( trect2, self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].Title, str1, regcardslot + 100);
                }
                else
                {
                  trect3 = Rectangle::new(x3, y6, num28 - 10, num29 - 10);
                  trect2 = trect3;
                  self.AddMouse( trect2, self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[regcardslot]].Title, str1, regcardslot + 100);
                }
              }
            }
          }
        }
      }
      bool flag1 = true;
      if (self.viewMode == 2 && self.categorySelectMode[self.currentCat] == 1 | self.categorySelectMode[self.currentCat] == 2)
      {
        let mut trect4: &Rectangle = &rectangle1
        bool tTempZones = false;
        if (self.currentCat == 7 | self.currentCat == 3)
          tTempZones = true;
        let mut tsubpart7: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, false, trect4.Width, trect4.Height, trealhex: true, tMapWidth: trect4.Width, tMapHeight: trect4.Height, ZoomLevel: 0, ttempValue4mustBe: self.miniSelectValue, tblockMapMove: true, tTempValue3usedForAlpha: true, tTempZones: tTempZones);
        self.miniId = self.AddSubPart( tsubpart7, trect4.X, trect4.Y, trect4.Width, trect4.Height, 0);
        if (self.categorySelectMode[self.currentCat] == 1)
          self.AddMouse( trect4, "", "Click to select a regime", 51);
        if (self.categorySelectMode[self.currentCat] == 2)
          self.AddMouse( trect4, "", "Click to select a zone", 51);
      }
      if (self.viewMode == 2)
      {
        let mut tlistselect1: i32 = -1;
        self.OptionsListObj = ListClass::new();
        self.OptionsList2Obj = ListClass::new();
        let mut num55: i32 = -1;
        let mut tlistsize1: i32 =  Math.Round(Math.Floor( rectangle3.Height / 24.0)) - 1;
        let mut tlistsize2: i32 =  Math.Round(Math.Floor( rectangle4.Height / 24.0)) - 1;
        tstring: String = "";
        if (self.currentCat == 1)
          tstring = "Select Major Regime";
        if (self.currentCat == 2)
          tstring = "Select Minor Regime";
        if (self.currentCat == 3)
          tstring = "Select Zone";
        if (self.currentCat == 4)
          tstring = "Nothing to Select";
        if (self.currentCat == 5)
          tstring = "Select Regime";
        if (self.currentCat == 6)
          tstring = "Select OHQ";
        if (self.currentCat == 7)
          tstring = "Select Zone";
        if (self.currentCat == 8)
          tstring = "Select Leader";
        if (self.currentCat == 9)
          tstring = "Select Unit";
        bool[] flagArray1 = new bool[self.maxMiniSelectValue + 1];
        if (self.categorySelectMode[self.currentCat] == 1)
        {
          let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index9: i32 = 0; index9 <= mapWidth; index9 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index10: i32 = 0; index10 <= mapHeight; index10 += 1)
            {
              if (self.game.EditObj.TempValue4[0].Value[index9, index10] > 0)
                flagArray1[self.game.EditObj.TempValue4[0].Value[index9, index10]] = true;
            }
          }
        }
        bool[] flagArray2 = new bool[self.maxMiniSelectValue + 10 + 1];
        if (self.categorySelectMode[self.currentCat] == 2)
        {
          let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
          for (let mut index11: i32 = 0; index11 <= mapWidth; index11 += 1)
          {
            let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
            for (let mut index12: i32 = 0; index12 <= mapHeight; index12 += 1)
            {
              if (self.game.EditObj.TempValue4[0].Value[index11, index12] > 0)
                flagArray2[self.game.EditObj.TempValue4[0].Value[index11, index12]] = true;
            }
          }
        }
        if (tstring.Length > 1)
          DrawMod.DrawTextColouredConsole( g, tstring, self.game.MarcFont5, rectangle3.X, rectangle3.Y - 18, self.game.seColWhite);
        if (self.categorySelectMode[self.currentCat] == 1 && self.game.Data.RegimeCounter > -1)
        {
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut tdata: i32 = 0; tdata <= regimeCounter; tdata += 1)
          {
            if (!self.game.Data.RegimeObj[tdata].hideFromList & flagArray1[self.game.Data.RegimeObj[tdata].id] && !self.game.Data.RegimeObj[tdata].Sleep | !self.game.Data.RegimeObj[tdata].DipBlock)
            {
              bool flag2 = true;
              index3 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.game.Data.RegimeObj[tdata].id, 1)));
              if (self.currentCat == 1 & index3 != 1)
                flag2 = false;
              if (self.currentCat == 2 & index3 != 2)
                flag2 = false;
              if (self.currentCat == 1 & self.game.Data.RegimeObj[tdata].Sleep)
                flag2 = false;
              if (flag2)
              {
                num55 += 1;
                if (self.miniSelectValue == self.game.Data.RegimeObj[tdata].id)
                  tlistselect1 = num55;
                name: String = self.game.Data.RegimeObj[tdata].Name;
                let mut num56: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById12].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[tdata].id, 2, "recon", 3)));
                tname: String;
                if (tdata == self.game.Data.Turn)
                  tname = "⍟ " + name;
                else if (num56 >= 2)
                {
                  switch ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.game.Data.RegimeObj[tdata].id, 1))))
                  {
                    case 1:
                      tname = "⌾ " + name;
                      break;
                    case 2:
                      let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.game.Data.RegimeObj[tdata].id, 2)));
                      tname =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById13].GetData(0, idValue, 1))) != num2 ? "∘ " + name : "∞ " + name;
                      break;
                    case 3:
                      tname = "⊷ " + name;
                      break;
                    default:
                      tname = "  " + name;
                      break;
                  }
                }
                else
                {
                  switch ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.game.Data.RegimeObj[tdata].id, 1))))
                  {
                    case 1:
                      tname = "⌾ " + name;
                      break;
                    case 2:
                      tname = "∘ " + name;
                      break;
                    case 3:
                      tname = "⊷ " + name;
                      break;
                    default:
                      tname = "  " + name;
                      break;
                  }
                }
                self.OptionsListObj.add(tname, tdata);
              }
            }
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart8: SubPartClass =  new ListSubPartClass(self.OptionsListObj, tlistsize1, rectangle3.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle3.X, bby: rectangle3.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsListId = self.AddSubPart( tsubpart8, rectangle3.X, rectangle3.Y, rectangle3.Width, (tlistsize1 + 1) * 24, 0);
          }
        }
        if (self.categorySelectMode[self.currentCat] == 2 && self.game.Data.RegimeCounter > -1)
        {
          let mut maxMiniSelectValue: i32 = self.maxMiniSelectValue;
          for (let mut tdata: i32 = 1; tdata <= maxMiniSelectValue; tdata += 1)
          {
            if (self.zoneVisible[tdata] & flagArray2[tdata])
            {
              num55 += 1;
              if (self.miniSelectValue == tdata)
                tlistselect1 = num55;
              self.OptionsListObj.add(self.zoneName[tdata], tdata);
            }
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart9: SubPartClass =  new ListSubPartClass(self.OptionsListObj, tlistsize1, rectangle3.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle3.X, bby: rectangle3.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsListId = self.AddSubPart( tsubpart9, rectangle3.X, rectangle3.Y, rectangle3.Width, (tlistsize1 + 1) * 24, 0);
          }
        }
        if (self.categorySelectMode[self.currentCat] == 3)
        {
          let mut num57: i32 = num55 + 1;
          self.OptionsList2Obj.add("All", 0);
          if (self.miniCatSelectValue == -1)
            self.miniCatSelectValue = 0;
          if (0 == self.miniCatSelectValue)
            tlistselect1 = num57;
          if (self.game.Data.UnitCounter > -1)
          {
            let mut maxMiniSelectValue: i32 = self.maxMiniSelectValue;
            for (let mut tdata: i32 = 0; tdata <= maxMiniSelectValue; tdata += 1)
            {
              if (tdata <= self.game.Data.UnitCounter && self.game.Data.UnitObj[tdata].IsHQ & self.game.Data.UnitObj[tdata].Regime == self.game.Data.Turn & self.game.Data.UnitObj[tdata].PreDef == -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 8)
              {
                num57 += 1;
                if (self.miniCatSelectValue == tdata)
                  tlistselect1 = num57;
                self.OptionsList2Obj.add(self.game.Data.UnitObj[tdata].Name, tdata);
              }
            }
            if (self.OptionsList2Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, tlistselect1);
              self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
            }
            else
            {
              let mut tsubpart10: SubPartClass =  new ListSubPartClass(self.OptionsList2Obj, tlistsize2, rectangle4.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle4.X, bby: rectangle4.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
              self.OptionsList2Id = self.AddSubPart( tsubpart10, rectangle4.X, rectangle4.Y, rectangle4.Width, (tlistsize2 + 1) * 24, 0);
            }
          }
          num55 = -1;
          tlistselect1 = -1;
          if (self.game.Data.UnitCounter > -1)
          {
            let mut maxMiniSelectValue: i32 = self.maxMiniSelectValue;
            for (let mut tdata: i32 = 0; tdata <= maxMiniSelectValue; tdata += 1)
            {
              if (tdata <= self.game.Data.UnitCounter && self.game.Data.UnitObj[tdata].TempUnitSelectable && self.game.Data.UnitObj[tdata].HQ == self.miniCatSelectValue | self.miniCatSelectValue == 0)
              {
                num55 += 1;
                if (self.miniUnitSelect == -1)
                  self.miniUnitSelect = tdata;
                if (self.miniUnitSelect == tdata)
                  tlistselect1 = num55;
                self.OptionsListObj.add(self.game.Data.UnitObj[tdata].Name, tdata);
              }
            }
            if (self.OptionsListId > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
              self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
            }
            else
            {
              let mut tsubpart11: SubPartClass =  new ListSubPartClass(self.OptionsListObj, tlistsize1, rectangle3.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle3.X, bby: rectangle3.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
              self.OptionsListId = self.AddSubPart( tsubpart11, rectangle3.X, rectangle3.Y, rectangle3.Width, (tlistsize1 + 1) * 24, 0);
            }
          }
        }
        if (self.categorySelectMode[self.currentCat] == 5)
        {
          let mut num58: i32 = num55 + 1;
          self.OptionsList2Obj.add("All", 0);
          if (self.miniCatSelectValue == -1)
            self.miniCatSelectValue = 0;
          if (0 == self.miniCatSelectValue)
            tlistselect1 = num58;
          if (self.game.Data.UnitCounter > -1)
          {
            let mut maxMiniSelectValue: i32 = self.maxMiniSelectValue;
            for (let mut tdata: i32 = 0; tdata <= maxMiniSelectValue; tdata += 1)
            {
              if (tdata <= self.game.Data.UnitCounter && self.game.Data.UnitObj[tdata].IsHQ & self.game.Data.UnitObj[tdata].Regime == self.game.Data.Turn & self.game.Data.UnitObj[tdata].PreDef == -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tdata].Historical].Type == 5)
              {
                num58 += 1;
                if (self.miniCatSelectValue == tdata)
                  tlistselect1 = num58;
                self.OptionsList2Obj.add(self.game.Data.UnitObj[tdata].Name, tdata);
              }
            }
            if (self.OptionsList2Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, tlistselect1);
              self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
            }
            else
            {
              let mut tsubpart12: SubPartClass =  new ListSubPartClass(self.OptionsList2Obj, tlistsize2, rectangle4.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle4.X, bby: rectangle4.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
              self.OptionsList2Id = self.AddSubPart( tsubpart12, rectangle4.X, rectangle4.Y, rectangle4.Width, (tlistsize2 + 1) * 24, 0);
            }
          }
          num55 = -1;
          tlistselect1 = -1;
          if (self.game.Data.UnitCounter > -1)
          {
            let mut maxMiniSelectValue: i32 = self.maxMiniSelectValue;
            for (let mut tdata: i32 = 0; tdata <= maxMiniSelectValue; tdata += 1)
            {
              if (tdata <= self.game.Data.UnitCounter && self.game.Data.UnitObj[tdata].TempUnitSelectable && self.game.Data.UnitObj[tdata].HQ == self.miniCatSelectValue | self.miniCatSelectValue == 0)
              {
                num55 += 1;
                if (self.miniUnitSelect == -1)
                  self.miniUnitSelect = tdata;
                if (self.miniUnitSelect == tdata)
                  tlistselect1 = num55;
                self.OptionsListObj.add(self.game.Data.UnitObj[tdata].Name, tdata);
              }
            }
            if (self.OptionsListId > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
              self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
            }
            else
            {
              let mut tsubpart13: SubPartClass =  new ListSubPartClass(self.OptionsListObj, tlistsize1, rectangle3.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle3.X, bby: rectangle3.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
              self.OptionsListId = self.AddSubPart( tsubpart13, rectangle3.X, rectangle3.Y, rectangle3.Width, (tlistsize1 + 1) * 24, 0);
            }
          }
        }
        if (self.categorySelectMode[self.currentCat] == 4)
        {
          if (self.miniSelectLeader > -1 & self.miniSelectValue > -1 & self.miniCatSelectValue == -1)
            self.miniCatSelectValue =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[self.miniSelectValue, 6]));
          if (self.miniCatSelectValue == -1)
            self.miniCatSelectValue = 0;
          let mut num59: i32 = num55 + 1;
          self.OptionsList2Obj.add("All", 0);
          if (0 == self.miniCatSelectValue)
            tlistselect1 = num59;
          let mut num60: i32 = num59 + 1;
          self.OptionsList2Obj.add("Reserve Pool", 1);
          if (1 == self.miniCatSelectValue)
            tlistselect1 = num60;
          let mut num61: i32 = num60 + 1;
          self.OptionsList2Obj.add("OHQ Commanders", 3);
          if (3 == self.miniCatSelectValue)
            tlistselect1 = num61;
          let mut num62: i32 = num61 + 1;
          self.OptionsList2Obj.add("SHQ Commanders", 4);
          if (4 == self.miniCatSelectValue)
            tlistselect1 = num62;
          let mut num63: i32 = num62 + 1;
          self.OptionsList2Obj.add("Directors", 5);
          if (5 == self.miniCatSelectValue)
            tlistselect1 = num63;
          let mut num64: i32 = num63 + 1;
          self.OptionsList2Obj.add("Advisors", 6);
          if (6 == self.miniCatSelectValue)
            tlistselect1 = num64;
          let mut num65: i32 = num64 + 1;
          self.OptionsList2Obj.add("Secretary", 8);
          if (8 == self.miniCatSelectValue)
            tlistselect1 = num65;
          let mut num66: i32 = num65 + 1;
          self.OptionsList2Obj.add("Governors", 10);
          if (10 == self.miniCatSelectValue)
            tlistselect1 = num66;
          if (self.OptionsList2Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, tlistselect1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
          }
          else
          {
            let mut tsubpart14: SubPartClass =  new ListSubPartClass(self.OptionsList2Obj, tlistsize2, rectangle4.Width, tlistselect1, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle4.X, bby: rectangle4.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
            self.OptionsList2Id = self.AddSubPart( tsubpart14, rectangle4.X, rectangle4.Y, rectangle4.Width, (tlistsize2 + 1) * 24, 0);
          }
          let mut num67: i32 = -1;
          let mut tlistselect2: i32 = -1;
          let mut tlistsize3: i32 =  Math.Round(Math.Floor( rectangle3.Height / 56.0)) - 1;
          let mut tValueWidth: i32 = rectangle3.Width - 200;
          index3 = rectangle3.Width - 400;
          if (index3 > 0)
            tValueWidth =  Math.Round( tValueWidth -  index3 / 3.0);
          if (self.detailnr > -1)
          {
            let mut length: i32 = self.game.Data.StringListObj[stringListById7].Length;
            for (let mut tdata: i32 = 0; tdata <= length; tdata += 1)
            {
              if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[tdata, 5])) == id1 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[tdata, 6])) > 0 && ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[tdata, 6])) == self.miniCatSelectValue | self.miniCatSelectValue == 0) &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[tdata, 6])) < 11)
              {
                if (true)
                {
                  index3 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[tdata, 0]));
                  tname: String = rectangle3.Width < 500 ? self.game.Data.StringListObj[stringListById7].Data[tdata, 3] + "\r\n" + self.game.Data.StringListObj[stringListById7].Data[tdata, 4] : self.game.Data.StringListObj[stringListById7].Data[tdata, 3] + " " + self.game.Data.StringListObj[stringListById7].Data[tdata, 4];
                  tvalue: String = rectangle3.Width < 400 ? self.game.EventRelatedObj.Helper_GetCharacterJobTitle(index3, shortJobSpecific: true) : self.game.EventRelatedObj.Helper_GetCharacterJobTitle(index3);
                  tvalue2: String = "Relation " + self.game.Data.StringListObj[stringListById7].Data[tdata, 20];
                  if (rectangle3.Width >= 550)
                    self.OptionsListObj.add(tname, tdata, tvalue, tvalue2, tbmp: ((Bitmap) self.game.CustomBitmapObj.DrawLeaderPortrait(index3, 40, 56).Clone()));
                  else
                    self.OptionsListObj.add(tname, tdata, tvalue, tbmp: ((Bitmap) self.game.CustomBitmapObj.DrawLeaderPortrait(index3, 40, 56).Clone()));
                }
              }
            }
          }
          self.OptionsListObj.Sort();
          let mut listCount: i32 = self.OptionsListObj.ListCount;
          for (let mut index13: i32 = 0; index13 <= listCount; index13 += 1)
          {
            num67 += 1;
            if (self.miniSelectValue == -1)
            {
              self.miniSelectValue = self.OptionsListObj.ListData[index13];
              self.miniSelectLeader =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].Data[self.miniSelectValue, 0]));
            }
            if (self.OptionsListObj.ListData[index13] == self.miniSelectValue)
              tlistselect2 = num67;
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect2);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart15: SubPartClass =  new ListSubPartClass(self.OptionsListObj, tlistsize3, rectangle3.Width, tlistselect2, self.game, tHeaderCenter: false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: rectangle3.X, bby: rectangle3.Y, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 56);
            self.OptionsListId = self.AddSubPart( tsubpart15, rectangle3.X, rectangle3.Y, rectangle3.Width, (tlistsize3 + 1) * 56, 0);
          }
        }
        if (self.categorySelectMode[self.currentCat] == 1)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
          self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
          if (self.miniSelectValue > -1)
          {
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEID", 1, self.miniSelectValue, true);
            let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(self.miniSelectValue);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMESLOT", 1, regimeById, true);
            let mut num68: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 2)));
            let mut setValue8: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById13].GetData(0, num68, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTURE", 1, setValue8, true);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTUREID", 1, num68, true);
            let mut setValue9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 13)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEAIID", 1, setValue9, true);
            index3 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regimeById];
            let mut num69: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, self.miniSelectValue, 1)));
            if (index3 == 0 & num69 == 2 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById12].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[regimeById].id, 2, "dipClear", 3))) < 1)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            else if (index3 == 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 1, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            let mut setValue10: i32 = 0;
            if (self.game.Data.RegimeObj[self.game.Data.Turn].AI)
              setValue10 = 1;
            self.game.Data.StringListObj[stringListById8].SetData(0, "SOURCEAI", 1, setValue10, true);
            let mut setValue11: i32 = 0;
            if (self.game.Data.RegimeObj[regimeById].AI)
              setValue11 = 1;
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETAI", 1, setValue11, true);
          }
        }
        else if (self.categorySelectMode[self.currentCat] == 2)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
          self.game.Data.StringListObj[stringListById8].SetData(0, "ZONEID", 1, self.miniSelectValue, true);
          self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
          if (self.miniSelectValue > -1)
          {
            let mut num70: i32 = self.zoneRegimeId[self.miniSelectValue];
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEID", 1, num70, true);
            let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(num70);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMESLOT", 1, regimeById, true);
            let mut num71: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num70, 2)));
            let mut setValue12: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById13].GetData(0, num71, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTURE", 1, setValue12, true);
            let mut setValue13: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num70, 1)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETMAJOR", 1, setValue13, true);
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCULTUREID", 1, num71, true);
            let mut setValue14: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num70, 13)));
            self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETREGIMEAIID", 1, setValue14, true);
            index3 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeRel[regimeById];
            let mut num72: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById11].GetData(0, num70, 1)));
            if (index3 == 0 & num72 == 2 &  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById12].GetData3(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, self.game.Data.RegimeObj[regimeById].id, 2, "dipClear", 3))) < 1)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
            else if (index3 == 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 1, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETWAR", 1, 0, true);
          }
        }
        else if (self.categorySelectMode[self.currentCat] == 3)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, self.miniUnitSelect, true);
          if (self.miniUnitSelect > 0)
          {
            if (self.game.Data.UnitObj[self.miniUnitSelect].Historical > 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.miniUnitSelect].Historical].ID, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
          }
          else
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        }
        else if (self.categorySelectMode[self.currentCat] == 5)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, self.miniUnitSelect, true);
          if (self.miniUnitSelect > 0)
          {
            if (self.game.Data.UnitObj[self.miniUnitSelect].Historical > 0)
              self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.miniUnitSelect].Historical].ID, true);
            else
              self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
          }
          else
            self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
        }
        else if (self.categorySelectMode[self.currentCat] == 4)
        {
          self.game.Data.StringListObj[stringListById8].SetData(0, "TARGETCHARID", 1, self.miniSelectLeader, true);
          self.game.Data.StringListObj[stringListById8].SetData(0, "HISID", 1, -1, true);
          self.game.Data.StringListObj[stringListById8].SetData(0, "UNITID", 1, -1, true);
        }
      }
      if (self.categorySelectMode[self.currentCat] > 0)
      {
        if (self.categorySelectMode[self.currentCat] == 1)
          index3 = self.game.HandyFunctionsObj.GetRegimeByID(self.miniSelectValue);
        else if (self.categorySelectMode[self.currentCat] == 2)
          index3 = self.miniSelectValue;
        else if (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5)
          index3 = self.miniUnitSelect;
        else if (self.categorySelectMode[self.currentCat] == 4)
          index3 = self.miniSelectLeader;
        let mut index14: i32 = self.hovernr;
        if (index14 == -1)
          index14 = self.detailnr;
        let mut index15: i32 = self.game.EditObj.TempValue4[0].Value[self.miniSelectX, self.miniSelectY];
        if (self.categorySelectMode[self.currentCat] == 1)
        {
          if (index14 > -1)
          {
            index14 = numArray4[index14];
            if (index15 > -1 & index15 < 9999 & index14 <= SL.Counter)
            {
              if (self.cardPlayable[index14, index15])
              {
                self.game.EventRelatedObj.Helper_Regime_Info_For_Cards(index3, "TARGET VALID", "", IO_ColNr.GreenDarker);
              }
              else
              {
                flag1 = false;
                self.game.EventRelatedObj.Helper_Regime_Info_For_Cards(index3, "NOT A VALID TARGET", self.cardWhyNot[index14, index15], IO_ColNr.Red);
              }
            }
            else
            {
              flag1 = false;
              self.game.EventRelatedObj.Helper_Regime_Info_For_Cards(index3, "NO TARGET", "", IO_ColNr.Red);
            }
          }
          else
            self.game.EventRelatedObj.Helper_Regime_Info_For_Cards(index3, "", "", IO_ColNr.Black);
        }
        if (self.categorySelectMode[self.currentCat] == 2)
        {
          if (index14 > -1)
          {
            index14 = numArray4[index14];
            if (index15 > -1 & index15 < 9999 & index14 <= SL.Counter)
            {
              if (self.cardPlayable[index14, index15])
              {
                self.game.EventRelatedObj.Helper_Zone_Info_For_Cards(index3, "TARGET VALID", "", IO_ColNr.GreenDarker);
              }
              else
              {
                flag1 = false;
                self.game.EventRelatedObj.Helper_Zone_Info_For_Cards(index3, "NOT A VALID TARGET", self.cardWhyNot[index14, index15], IO_ColNr.Red);
              }
            }
            else
            {
              flag1 = false;
              self.game.EventRelatedObj.Helper_Zone_Info_For_Cards(index3, "NO TARGET", "", IO_ColNr.Red);
            }
          }
          else
            self.game.EventRelatedObj.Helper_Zone_Info_For_Cards(index3, "", "", IO_ColNr.Black);
        }
        if (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5)
        {
          let mut miniUnitSelect: i32 = self.miniUnitSelect;
          if (index14 > -1)
          {
            index14 = numArray4[index14];
            if (miniUnitSelect > -1 & index14 <= SL.Counter)
            {
              if (self.cardPlayable[index14, miniUnitSelect])
              {
                self.game.EventRelatedObj.Helper_Unit_Info_For_Cards(index3, "TARGET VALID", "", IO_ColNr.GreenDarker);
              }
              else
              {
                flag1 = false;
                self.game.EventRelatedObj.Helper_Unit_Info_For_Cards(index3, "NOT A VALID TARGET", self.cardWhyNot[index14, miniUnitSelect], IO_ColNr.Red);
              }
            }
            else
            {
              flag1 = false;
              self.game.EventRelatedObj.Helper_Unit_Info_For_Cards(index3, "NO TARGET", "", IO_ColNr.Red);
            }
          }
          else
            self.game.EventRelatedObj.Helper_Unit_Info_For_Cards(index3, "", "", IO_ColNr.Black);
        }
        if (self.categorySelectMode[self.currentCat] == 4)
        {
          let mut miniSelectLeader: i32 = self.miniSelectLeader;
          let mut row: i32 = self.game.Data.StringListObj[stringListById7].FindRow(0, miniSelectLeader);
          if (index14 > -1)
          {
            let mut index16: i32 = numArray4[index14];
            if (miniSelectLeader > -1 & index16 <= SL.Counter)
            {
              if (self.cardPlayable[index16, row])
              {
                self.game.EventRelatedObj.Helper_Character_Info_For_Cards(index3, "TARGET VALID", "", IO_ColNr.GreenDarker);
              }
              else
              {
                flag1 = false;
                self.game.EventRelatedObj.Helper_Character_Info_For_Cards(index3, "NOT A VALID TARGET", self.cardWhyNot[index16, row], IO_ColNr.Red);
              }
            }
            else
            {
              flag1 = false;
              self.game.EventRelatedObj.Helper_Character_Info_For_Cards(index3, "NO TARGET", "", IO_ColNr.Red);
            }
          }
          else
            self.game.EventRelatedObj.Helper_Character_Info_For_Cards(index3, "", "", IO_ColNr.Black);
        }
      }
      else
      {
        let mut index17: i32 = self.hovernr;
        if (index17 == -1)
          index17 = self.detailnr;
        if (index17 > -1)
        {
          let mut index18: i32 = numArray4[index17];
          if (index18 <= SL.Counter)
          {
            if (!self.cardPlayable[index18, 1])
              flag1 = false;
          }
          else
            flag1 = false;
        }
      }
      if (self.viewMode == 2 && self.categorySelectMode[self.currentCat] > 0)
      {
        tTexty1: String = self.game.EventRelatedObj.CheckKey(num1, "FINALTEXT", 2, 0);
        let mut x9: i32 = rectangle2.X;
        let mut y12: i32 = rectangle2.Y;
        let mut width: i32 = rectangle2.Width;
        let mut height: i32 = rectangle2.Height;
        let mut num73: i32 =  Math.Round( (rectangle2.Width - 324) / 2.0);
        UDSPartClass udsPartClass = new UDSPartClass(self.game, width, height, tTexty1,  self.OwnBitmap, x9, y12, true, true, tAlwaysShowBackground: true);
        let mut num74: i32 = udsPartClass.DoJustCheckHeight(true) + 30;
        let mut num75: i32 =  Math.Round( (height - num74) / 2.0);
        udsPartClass.Dispose();
        if (rectangle2.Width > 564)
        {
          self.game.EventRelatedObj.ExecKey(num1, "FINALTEXT", "", "", "");
          if (self.categorySelectMode[self.currentCat] == 4 & self.miniSelectLeader > -1)
          {
            let mut eventByLib: i32 = self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 531, 0, 0);
            self.game.EventRelatedObj.Helper_AddPortraitUdsCode(self.miniSelectLeader, -120, 50 - num75, 100, 140, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0, "CHARID", self.miniSelectLeader, eventByLib, mouseOverText: "Click for more information");
          }
          if ((self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5) & self.miniUnitSelect > 0 & self.miniUnitSelect <= self.game.Data.UnitCounter)
            self.game.EventRelatedObj.IO_AddUnitPic(-90, 80 - num75, 0, self.miniUnitSelect);
          if (self.categorySelectMode[self.currentCat] == 2 | self.categorySelectMode[self.currentCat] == 1 && self.miniSelectValue > 0)
          {
            let mut index19: i32 = -1;
            if (self.categorySelectMode[self.currentCat] == 1)
              index19 = self.game.HandyFunctionsObj.GetRegimeByID(self.miniSelectValue);
            if (self.categorySelectMode[self.currentCat] == 2)
              index19 = self.game.HandyFunctionsObj.GetRegimeByID(self.zoneRegimeId[self.miniSelectValue]);
            let mut x10: i32 = -120;
            let mut y13: i32 = 20 - num75;
            let mut bannerSpriteNr: i32 = self.game.Data.RegimeObj[index19].BannerSpriteNr;
            self.game.EventRelatedObj.IO_AddEventPic(x10, y13, 124, self.game.Data.RegimeObj[index19].Red, self.game.Data.RegimeObj[index19].Green, self.game.Data.RegimeObj[index19].Blue,  byte.MaxValue, bannerSpriteNr + 10000);
            let mut bannerSpriteNr2: i32 = self.game.Data.RegimeObj[index19].BannerSpriteNr2;
            if (bannerSpriteNr2 > 0)
              self.game.EventRelatedObj.IO_AddEventPic(x10, y13, 124, self.game.Data.RegimeObj[index19].Red2, self.game.Data.RegimeObj[index19].Green2, self.game.Data.RegimeObj[index19].Blue2,  byte.MaxValue, bannerSpriteNr2 + 10000);
            let mut symbolSpriteNr: i32 = self.game.Data.RegimeObj[index19].SymbolSpriteNr;
            if (symbolSpriteNr > 0)
              self.game.EventRelatedObj.IO_AddEventPic(x10 + 32, y13 + 55, 66, self.game.Data.RegimeObj[index19].Red3, self.game.Data.RegimeObj[index19].Green3, self.game.Data.RegimeObj[index19].Blue3,  byte.MaxValue, symbolSpriteNr + 10000);
            self.game.EventRelatedObj.IO_AddEventPic(x10, y13, 124, 220, -1, self.game.Data.RegimeObj[index19].Name + " banner", -1);
          }
          tTexty1 += self.game.EventRelatedObj.CheckKey(num1, "FINALTEXT", 2, 0);
        }
        tTexty2: String = "[element][type]layout[/type][w]" + num73.ToString() + "[/w][h]" + num75.ToString() + "[/h][/element]" + tTexty1;
        let mut tsubpart16: SubPartClass =  new UDSPartClass(self.game, width, height, tTexty2,  self.OwnBitmap, x9, y12, true, tAlwaysShowBackground: true);
        self.pageId = self.AddSubPart( tsubpart16, x9, y12, width, height, 0);
      }
      let mut num76: i32 = self.w - 335;
      let mut num77: i32 = 54 + Math.Max(0,  Math.Round( (self.h - 154 - 544) / 2.0));
      let mut num78: i32 = 300;
      let mut num79: i32 = 285;
      let mut num80: i32 =  Math.Round( num76 +  (num78 - 190) / 2.0 - 10.0);
      let mut num81: i32 = num77 - 5;
      let mut num82: i32 = 235;
      let mut num83: i32 = num80 - 10;
      let mut num84: i32 = num81 + 320;
      if (self.scrapMode)
      {
        let mut y14: i32 =  Math.Round( self.h / 2.0 - 150.0);
        let mut num85: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, id1, 1, "scrapPoints", 2)));
        tstring3: String = "You are in";
        DrawMod.DrawTextColouredConsoleCenter( g, tstring3, self.game.MarcFont4, num76 +  Math.Round( num78 / 2.0), y14, self.game.seColWhite);
        let mut y15: i32 = y14 + 20;
        tstring4: String = "Scrap Mode";
        DrawMod.DrawTextColouredConsoleCenter( g, tstring4, self.game.MarcFont3, num76 +  Math.Round( num78 / 2.0), y15, self.game.seColWhite);
        let mut y16: i32 = y15 + 40;
        tstring5: String = "You currently have";
        DrawMod.DrawTextColouredConsoleCenter( g, tstring5, self.game.MarcFont4, num76 +  Math.Round( num78 / 2.0), y16, self.game.seColWhite);
        let mut y17: i32 = y16 + 20;
        tstring6: String = num85.ToString() + " Scrap Points";
        DrawMod.DrawTextColouredConsoleCenter( g, tstring6, self.game.MarcFont2, num76 +  Math.Round( num78 / 2.0), y17, self.game.seColWhite);
        num77 = y17 + 40 + 20;
        let mut num86: i32 = num77;
        let mut num87: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, id1, 1, "scrapPointCost", 2)));
        if (num87 < 15)
          num87 = 15;
        if (num85 >= num87)
        {
          let mut tsubpart17: SubPartClass =  new TextButtonPartClass("CRAFT STRATAGEM [" + num87.ToString() + "sp]", num82, "Click to craft a Scrap Stratagem for " + num87.ToString() + " Scrap Points.",  self.OwnBitmap, num83, num86, theight: 60, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.buyScrapId = self.AddSubPart( tsubpart17, num83, num86, num82, 40, 1);
        }
        else
        {
          let mut tsubpart18: SubPartClass =  new TextButtonPartClass("CRAFT STRATAGEM [" + num87.ToString() + "sp]", num82, "Sorry, but you do not have the required " + num87.ToString() + " Scrap Points to craft a Scrap Stratagem. Scrap some more Stratagems to gain more Scrap Points!",  self.OwnBitmap, num83, num86, true, theight: 60, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.buyScrapId2 = self.AddSubPart( tsubpart18, num83, num86, num82, 40, 1);
        }
      }
      if (!self.scrapMode)
      {
        let mut num88: i32 = num77 + 300;
        let mut num89: i32 = num79 - 300;
        str4: String = "PP";
        bool flag3 = false;
        if (self.detailnr > -1)
        {
          if (self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].customCostType == 1)
          {
            if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "fp", 2))) >= self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].customCostQty)
              flag3 = true;
            str4 = "FP";
          }
          else if (self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >= self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].PPCost | self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].PPCost == 0)
            flag3 = true;
        }
        if (self.hovernr > -1)
        {
          let mut nr: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.hovernr];
           let mut local19: &Graphics = &g;
          bitmap = self.game.CustomBitmapObj.DrawActionCardSe1(self.game.Data.Turn, nr);
           let mut local20: &Bitmap = &bitmap;
          let mut x: i32 = num80 + 10;
          let mut y: i32 = num81 + 10;
          DrawMod.DrawSimple( local19,  local20, x, y);
          ttext: String;
          if (self.rememberExtraS.Length > 1)
          {
            if (self.game.Data.ActionCardObj[nr].MouseOver.Length > 1)
              ttext = self.game.Data.ActionCardObj[nr].MouseOver + "\r\n\r\n" + "Effect when played:".ToUpper() + "\r\n" + self.rememberExtraS;
            else
              ttext = "Effect when played:".ToUpper() + "\r\n" + self.rememberExtraS;
          }
          else
            ttext = self.game.Data.ActionCardObj[nr].MouseOver.Length <= 1 ? "No requirements to play Stratagem" : self.game.Data.ActionCardObj[nr].MouseOver;
          trect3 = Rectangle::new(num80 + 10, num81 + 10, 190, 266);
          let mut trect5: &Rectangle = &trect3
          self.AddMouse( trect5, "REGIMECARD", ttext);
        }
        else if (self.detailnr > -1)
        {
          let mut nr: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr];
           let mut local21: &Graphics = &g;
          bitmap = self.game.CustomBitmapObj.DrawActionCardSe1(self.game.Data.Turn, nr);
           let mut local22: &Bitmap = &bitmap;
          let mut x: i32 = num80 + 10;
          let mut y: i32 = num81 + 10;
          DrawMod.DrawSimple( local21,  local22, x, y);
          if (Information.IsNothing( self.game.Data.ActionCardObj[nr].MouseOver))
            self.game.Data.ActionCardObj[nr].MouseOver = "";
          if (Information.IsNothing( self.rememberExtraS))
            self.rememberExtraS = "";
          ttext: String;
          if (self.rememberExtraS.Length > 1)
          {
            if (self.game.Data.ActionCardObj[nr].MouseOver.Length > 1)
              ttext = "Requirements:\r\n" + self.game.Data.ActionCardObj[nr].MouseOver + "\r\n\r\n" + "Effect when played:".ToUpper() + "\r\n" + self.rememberExtraS;
            else
              ttext = "Effect when played:".ToUpper() + "\r\n" + self.rememberExtraS;
          }
          else
            ttext = self.game.Data.ActionCardObj[nr].MouseOver.Length <= 1 ? "No requirements to play Stratagem" : "Requirements:\r\n" + self.game.Data.ActionCardObj[nr].MouseOver;
          trect3 = Rectangle::new(num80 + 10, num81 + 10, 190, 266);
          let mut trect6: &Rectangle = &trect3
          self.AddMouse( trect6, "REGIMECARD", ttext);
        }
        let mut x11: i32 = num80 - 35;
        let mut y18: i32 = num81 + 290;
        let mut width: i32 = 275;
        bool flag4 = false;
        tstring7: String = "TARGET";
        tstring8: String = "xxxx";
        if (self.categorySelectMode[self.currentCat] == 0)
        {
          tstring7 = "TARGET REGIME";
          tstring8 = "Your own Regime";
        }
        if (self.categorySelectMode[self.currentCat] == 1)
        {
          tstring7 = "TARGET REGIME";
          if (self.miniSelectValue == -1)
          {
            tstring8 = "No Regime selected";
            flag4 = true;
          }
          else
            tstring8 = self.game.Data.RegimeObj[self.game.HandyFunctionsObj.GetRegimeByID(self.miniSelectValue)].Name;
        }
        if (self.categorySelectMode[self.currentCat] == 2)
        {
          tstring7 = "TARGET ZONE";
          if (self.miniSelectValue == -1)
          {
            tstring8 = "No Zone selected";
            flag4 = true;
          }
          else
            tstring8 = self.zoneName[self.miniSelectValue];
        }
        if (self.categorySelectMode[self.currentCat] == 3)
        {
          tstring7 = "TARGET OHQ";
          if (self.miniUnitSelect == -1)
          {
            tstring8 = "No OHQ selected";
            flag4 = true;
          }
          else
            tstring8 = self.game.Data.UnitObj[self.miniUnitSelect].Name;
        }
        if (self.categorySelectMode[self.currentCat] == 4)
        {
          tstring7 = "TARGET LEADER";
          if (self.miniSelectLeader == -1)
          {
            tstring8 = "No Leader selected";
            flag4 = true;
          }
          else
            tstring8 = self.game.Data.StringListObj[stringListById7].GetData(0, self.miniSelectLeader, 3) + " " + self.game.Data.StringListObj[stringListById7].GetData(0, self.miniSelectLeader, 4);
        }
        if (self.categorySelectMode[self.currentCat] == 5)
        {
          tstring7 = "TARGET UNIT";
          if (self.miniUnitSelect == -1)
          {
            tstring8 = "No Unit selected";
            flag4 = true;
          }
          else
            tstring8 = self.game.Data.UnitObj[self.miniUnitSelect].Name;
        }
        if (self.detailnr > -1)
        {
          DrawMod.DrawTextColouredConsoleCenter( g, tstring7, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
          let mut y19: i32 = y18 + 16;
          num90: i32;
          if (flag4)
          {
            DrawMod.DrawTextColouredConsoleCenter( g, tstring8, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y19, self.game.seColRed);
            num90 = y19 + 16;
          }
          else
          {
            if (self.h > 742 & self.categorySelectMode[self.currentCat] == 4)
            {
               let mut local23: &Graphics = &g;
              bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(self.miniSelectLeader, 50, 70, true);
               let mut local24: &Bitmap = &bitmap;
              let mut x12: i32 = x11 +  Math.Round( width / 2.0) - 25;
              let mut y20: i32 = y19;
              DrawMod.DrawSimple( local23,  local24, x12, y20);
              trect3 = Rectangle::new(x11 +  Math.Round( width / 2.0) - 25, y19, 50, 70);
              let mut trect7: &Rectangle = &trect3
              self.AddMouse( trect7, "", "Click for more information", 11, self.miniSelectLeader);
              y19 += 70;
            }
            DrawMod.DrawTextColouredConsoleCenter( g, tstring8, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y19, self.game.seColWhite);
            num90 = y19 + 16;
          }
          y18 = num90 + 10;
        }
        tstring9: String = "EXECUTED BY";
        let mut num91: i32 = -1;
        let mut index20: i32 = self.hovernr;
        if (index20 == -1)
          index20 = self.detailnr;
        if (index20 > -1)
          num91 = self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[index20]].TempVar0;
        if (num91 > -1)
        {
          let mut num92: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num91, 13)));
          let mut idValue3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num91, 5)));
          let mut idValue4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num91, 14)));
          let mut num93: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num91, 7)));
          let mut index21: i32 = stringListById3;
          if (num93 == 1)
          {
            let mut regimeById: i32 = self.game.HandyFunctionsObj.GetRegimeByID(self.game.EditObj.TempValue4[0].Value[self.miniSelectX, self.miniSelectY]);
            if (regimeById > -1 && !self.game.Data.RegimeObj[regimeById].AI & num92 == 1)
              index21 = stringListById4;
          }
          let mut num94: i32 = -1;
          SimpleList simpleList3 = SimpleList::new();
          bool flag5 = false;
          self.game.EventRelatedObj.fixedAvgRollOverule = true;
          str5: String;
          if (flag1)
          {
            String2: String = "[" + num91.ToString() + "]";
            self.game.Data.StringListObj[stringListById8].SetData(0, "DIFF", 1, 0);
            let mut length: i32 = self.game.Data.StringListObj[index21].Length;
            for (let mut index22: i32 = 0; index22 <= length; index22 += 1)
            {
              if (Strings.InStr(self.game.Data.StringListObj[index21].Data[index22, 0], String2) > 0)
              {
                str6: String = self.game.Data.StringListObj[index21].Data[index22, 1];
                str7: String = self.game.Data.StringListObj[index21].Data[index22, 2];
                if (str7.Length > 1)
                {
                  if ((uint) Strings.InStr(str7, "DIFF=") > 0U)
                  {
                    let mut num95: i32 = 1;
                    Random random;
                    if (str6.Length > 1)
                    {
                      eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
                      let mut id2: i32 = self.game.Data.StringListObj[stringListById9].ID;
                      let mut id3: i32 = self.game.Data.StringListObj[stringListById8].ID;
                      logicString: String = str6;
                      random = (Random) null;
                       Random local =  random;
                      num95 = eventRelatedObj.CheckLogicStringStart(id2, id3, logicString, 0,  local);
                    }
                    if (num95 > 0)
                    {
                      if (Strings.InStr(str7, "dth(") > 0)
                      {
                        flag5 = true;
                        let mut Start1: i32 = Strings.InStr(str7, "dth(");
                        let mut Start2: i32 = Strings.InStr(Start1, str7, ",");
                        let mut num96: i32 = Strings.InStr(Start2, str7, ")");
                        str5 = !(Start2 < num96 & num96 > 0 & Start2 > 0) ? "Random Roll" : Strings.Mid(str7, Start1 + 4, Start2 - (Start1 + 4)) + "d" + Strings.Mid(str7, Start2 + 1, num96 - (Start2 + 1));
                      }
                      else
                      {
                        eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
                        let mut id4: i32 = self.game.Data.StringListObj[stringListById9].ID;
                        let mut id5: i32 = self.game.Data.StringListObj[stringListById8].ID;
                        logicString: String = str7;
                        random = (Random) null;
                         Random local =  random;
                        eventRelatedObj.ExecSetLogicWithReturnList(id4, id5, logicString,  local);
                      }
                    }
                  }
                  str8: String = str7.ToLower().Replace(" ", "");
                  if (Strings.InStr(str8, "che(249,1,charid") > 0)
                  {
                    let mut Start: i32 = Strings.InStr(Strings.InStr(Strings.InStr(str8, "che(249,1,charid"), str8, "charid"), str8, ",");
                    let mut num97: i32 = Strings.InStr(Start, str8, ")");
                    InputStr: String = Strings.Mid(str8, Start + 1, num97 - Start - 1);
                    simpleList3.AddWeight( Math.Round(Conversion.Val(InputStr)), 1, 1);
                  }
                  if (Strings.InStr(str8, "che(249,1,targetcharid") > 0)
                  {
                    let mut Start: i32 = Strings.InStr(Strings.InStr(Strings.InStr(str8, "che(249,1,targetcharid"), str8, "targetcharid"), str8, ",");
                    let mut num98: i32 = Strings.InStr(Start, str8, ")");
                    InputStr: String = Strings.Mid(str8, Start + 1, num98 - Start - 1);
                    simpleList3.AddWeight( Math.Round(Conversion.Val(InputStr)), 1, 2);
                  }
                }
              }
            }
            num94 = Conversions.ToInteger(self.game.Data.StringListObj[stringListById8].GetData(0, "DIFF", 1));
            if (num94 > 999)
              num94 = 999;
          }
          self.game.EventRelatedObj.fixedAvgRollOverule = false;
          let mut num99: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData2(1, idValue3, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
          let mut num100: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue3, 2)));
          if (num99 < 1)
          {
            num99 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData2(1, idValue4, 2, self.game.Data.RegimeObj[self.game.Data.Turn].id, 0)));
            num100 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue4, 2)));
          }
          let mut num101: i32 = -1;
          if (num99 > 0)
          {
            switch (num100)
            {
              case 1:
                num101 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 5, num99, -1);
                break;
              case 2:
                num101 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 9, num99, -1);
                break;
            }
          }
          if (num93 == 2 & self.miniSelectValue > -1)
          {
            let mut characterId: i32 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 10, self.miniSelectValue, -1);
            if (characterId > 0 & idValue3 < 1)
              num101 = characterId;
          }
          if (num101 < 1)
            num101 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[self.game.Data.Turn].id, 8, 0, -1);
          self.game.Data.StringListObj[stringListById8].SetData(0, "ORGID", 1, num99, true);
          if (num101 > 0)
            self.game.Data.StringListObj[stringListById8].SetData(0, "CHARID", 1, num101, true);
          tstring10: String = "Uknown";
          tstring11: String = "Unknown";
          if (num101 > 0)
          {
            tstring11 = self.game.EventRelatedObj.Helper_GetCharacterJobTitle(num101, shortJobSpecific: true);
            tstring10 = self.game.Data.StringListObj[stringListById7].GetData(0, num101, 3) + " " + self.game.Data.StringListObj[stringListById7].GetData(0, num101, 4);
            setValue: String = tstring10;
            self.game.Data.StringListObj[stringListById8].SetData(0, "CHARNAME", 1, setValue, true);
          }
          if (self.detailnr > -1)
          {
            DrawMod.DrawTextColouredConsoleCenter( g, tstring9, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
            let mut y21: i32 = y18 + 16;
            if (self.h > 672)
            {
               let mut local25: &Graphics = &g;
              bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(num101, 50, 70, true);
               let mut local26: &Bitmap = &bitmap;
              let mut x13: i32 = x11 +  Math.Round( width / 2.0) - 25;
              let mut y22: i32 = y21;
              DrawMod.DrawSimple( local25,  local26, x13, y22);
              trect3 = Rectangle::new(x11 +  Math.Round( width / 2.0) - 25, y21, 50, 70);
              let mut trect8: &Rectangle = &trect3
              self.AddMouse( trect8, "", "Click for more information", 11, num101);
              y21 += 70;
            }
            DrawMod.DrawTextColouredConsoleCenter( g, tstring10, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y21, self.game.seColWhite);
            DrawMod.DrawTextColouredConsoleCenter( g, tstring11, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y21 + 16, self.game.seColWhite);
            y18 = y21 + 42;
          }
          if (!flag1 & self.miniSelectValue < 0)
          {
            tstring12: String = "IMPOSSIBLE";
            str9: String = "";
            if (SL.FindNr(num91) > -1)
              str9 = self.cardWhyNot[SL.FindNr(num91), 1];
            if (Information.IsNothing( str9))
              str9 = "There is a reason this is impossible, but we cannot tell you.";
            if (Operators.CompareString(str9, "", false) == 0)
              str9 = "There is a reason this is impossible, but we cannot tell you.";
            DrawMod.DrawTextColouredConsoleCenter( g, tstring12, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
            let mut y23: i32 = y18 + 16;
            if (!Information.IsNothing( str9))
            {
              SizeF sizeF1 = SizeF::new();
              StringFormat stringFormat = StringFormat::new();
              SizeF sizeF2 = g.MeasureString(str9, self.game.MarcFont4, width - 20);
              DrawMod.DrawTextColouredConsoleMultiline( g, str9, self.game.MarcFont4, x11 + 10, y23, self.game.seColRed, width - 20, 80, true);
              y23 =  Math.Round( ( y23 + sizeF2.Height));
            }
            y18 = y23 + 10;
          }
          if (!flag1 & !flag4 & SL.FindNr(num91) > -1 & self.miniSelectValue > -1)
          {
            tstring13: String = "INVALID TARGET";
            str10: String = "";
            if (SL.FindNr(num91) > -1)
              str10 = self.cardWhyNot[SL.FindNr(num91), self.miniSelectValue];
            if (Information.IsNothing( str10))
              str10 = "Invalid Target";
            if (Operators.CompareString(str10, "", false) == 0)
              str10 = "Invalid Target";
            DrawMod.DrawTextColouredConsoleCenter( g, tstring13, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
            let mut y24: i32 = y18 + 16;
            if (!Information.IsNothing( str10))
            {
              SizeF sizeF3 = SizeF::new();
              StringFormat stringFormat = StringFormat::new();
              SizeF sizeF4 = g.MeasureString(str10, self.game.MarcFont4, width - 20);
              DrawMod.DrawTextColouredConsoleMultiline( g, str10, self.game.MarcFont4, x11 + 10, y24, self.game.seColRed, width - 20, 80, true);
              y24 =  Math.Round( ( y24 + sizeF4.Height));
            }
            y18 = y24 + 10;
          }
          if (flag1 & !flag4)
          {
            tstring14: String = "DIFFICULTY";
            tstring15: String;
            if (flag4)
              tstring15 = "No target selected";
            else if (!flag1)
              tstring15 = "No valid target";
            else if (num94 > 0 | flag5)
            {
              tstring15 = num94.ToString();
              if (flag5 & num94 > 0)
                tstring15 = str5 + " +" + num94.ToString();
              else if (flag5 & num94 < 1)
                tstring15 = str5;
            }
            else
              tstring15 = "No Tests";
            DrawMod.DrawTextColouredConsoleCenter( g, tstring14, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
            DrawMod.DrawTextColouredConsoleCenter( g, tstring15, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18 + 16, self.game.seColWhite);
            y18 += 42;
            if (!flag4 & flag1 && simpleList3.Counter > -1)
            {
              bool flag6 = false;
              bool flag7 = false;
              let mut counter3: i32 = simpleList3.Counter;
              for (let mut index23: i32 = 0; index23 <= counter3; index23 += 1)
              {
                if (simpleList3.Data1[index23] == 2)
                  flag6 = true;
                else
                  flag7 = true;
              }
              tstring16: String = flag6 ? (!(flag6 & !flag7) ? "ROLLS (BOTH LEADERS)" : "ROLLS FOR TARGET LEADER") : "ROLLS";
              DrawMod.DrawTextColouredConsoleCenter( g, tstring16, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y18, self.game.seColWhite);
              let mut y25: i32 = y18 + 16;
              let mut counter4: i32 = simpleList3.Counter;
              for (let mut index24: i32 = 0; index24 <= counter4; index24 += 1)
              {
                data: String = self.game.Data.StringListObj[stringListById10].GetData(0, simpleList3.Id[index24], 1);
                let mut num102: i32 = self.game.EventRelatedObj.Helper_SkillTotal(num101, simpleList3.Id[index24]);
                tstring17: String = num102 <= 0 ? (num102 != 0 ? data + " Skill Roll =  1d100 " + num102.ToString() : data + " Skill Roll =  1d100") : data + " Skill Roll = 1d100 + " + num102.ToString();
                bool flag8 = false;
                if (num101 > 0 & simpleList3.Data1[index24] != 2 &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById7].GetData(0, num101, 6))) == 8)
                {
                  tstring17 += " -25";
                  flag8 = true;
                }
                if (simpleList3.Data1[index24] == 2 | flag8)
                  tstring17 += " *";
                DrawMod.DrawTextColouredConsoleCenter( g, tstring17, self.game.MarcFont4, x11 +  Math.Round( width / 2.0), y25, self.game.seColWhite);
                if (simpleList3.Data1[index24] == 2)
                {
                  trect3 = Rectangle::new(x11, y25 - 20, width, 40);
                  let mut trect9: &Rectangle = &trect3
                  self.AddMouse( trect9, "ASTERIX *", "This *-sign means the Skill Roll is actually made by the targetted Leader.");
                }
                else if (flag8)
                {
                  trect3 = Rectangle::new(x11, y25 - 20, width, 40);
                  let mut trect10: &Rectangle = &trect3
                  self.AddMouse( trect10, "ASTERIX *", "This *-sign means because Secretary is executing the Stratagem there is a -25 penalty.");
                }
                y25 += 16;
              }
              y18 = y25 + 10;
            }
          }
        }
        let mut num103: i32 = num80 - 10;
        let mut num104: i32 = y18 + 2;
        let mut num105: i32 = 235;
        if (flag4)
          flag1 = false;
        if (self.hovernr > -1)
        {
          if (!flag1)
          {
            let mut tsubpart19: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "The selected target is not valid.",  self.OwnBitmap, num103, num104, true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.info2id = self.AddSubPart( tsubpart19, num103, num104, num105, 40, 1);
          }
          else if (flag3)
          {
            let mut tsubpart20: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "Click to play this stratagem",  self.OwnBitmap, num103, num104, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.Info1Id = self.AddSubPart( tsubpart20, num103, num104, num105, 40, 1);
          }
          else
          {
            let mut tsubpart21: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "You dont have the " + str4 + " to play this stratagem.",  self.OwnBitmap, num103, num104, true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.info2id = self.AddSubPart( tsubpart21, num103, num104, num105, 40, 1);
          }
        }
        else if (self.detailnr > -1)
        {
          if (!flag1)
          {
            let mut tsubpart22: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "The selected target is not valid.",  self.OwnBitmap, num103, num104, true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.info2id = self.AddSubPart( tsubpart22, num103, num104, num105, 40, 1);
          }
          else if (flag3)
          {
            let mut tsubpart23: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "Click to play this stratagem",  self.OwnBitmap, num103, num104, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.Info1Id = self.AddSubPart( tsubpart23, num103, num104, num105, 40, 1);
          }
          else
          {
            let mut tsubpart24: SubPartClass =  new TextButtonPartClass("EXECUTE STRATAGEM", num105, "You dont have the " + str4 + " to play this stratagem.",  self.OwnBitmap, num103, num104, true, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.info2id = self.AddSubPart( tsubpart24, num103, num104, num105, 40, 1);
          }
        }
      }
      self.CopyToEditObj();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            if (self.SubPartID[index] == self.pageId)
            {
              self.game.EditObj.TipButton = false;
              self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
              if (self.game.EditObj.TipButton)
                return;
            }
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
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          if (self.MouseData[mouseCounter] > -1)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[mouseCounter];
          self.game.EditObj.TipText = self.MouseText[mouseCounter];
          break;
        }
      }
    }

    pub handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (self.SubPartCounter > -1)
      {
        for (let mut subPartCounter: i32 = self.SubPartCounter; subPartCounter >= 0; subPartCounter += -1)
        {
          if (x > self.SubPartX[subPartCounter] & y > self.SubPartY[subPartCounter] & x < self.SubPartX[subPartCounter] + self.SubPartW[subPartCounter] & y < self.SubPartY[subPartCounter] + self.SubPartH[subPartCounter])
          {
            let mut subPart: SubPartClass = self.SubPartList[subPartCounter];
            let mut x1: i32 = x - self.SubPartX[subPartCounter];
            let mut y1: i32 = y - self.SubPartY[subPartCounter];
            WindowClass windowClass = (WindowClass) this;
             WindowClass local =  windowClass;
            if (subPart.HandleTimerWheel(x1, y1,  local))
            {
              windowReturnClass.SetFlag(true);
              if (self.miniId == self.SubPartID[subPartCounter])
              {
                if (self.miniId > 0)
                {
                  self.RemoveSubPart(self.miniId);
                  self.miniId = 0;
                }
                if (self.Info1Id > 0)
                {
                  self.RemoveSubPart(self.Info1Id);
                  self.Info1Id = 0;
                }
                self.game.EditObj.MiniMap = new Bitmap(10, 10);
                self.DoRefresh();
                self.prepareTempValue4();
                self.dostuff();
              }
              return windowReturnClass;
            }
          }
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.mouseOverWhichTab > 0 && !self.MouseInThisWindow)
      {
        self.mouseOverWhichTab = 0;
        self.dostuff();
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (self.game.EditObj.WINDOW_DEBUG_MODE)
      {
        let mut index1: i32 = -1;
        let mut mouseCounter: i32 = self.MouseCounter;
        for (let mut index2: i32 = 0; index2 <= mouseCounter; index2 += 1)
        {
          if (self.MouseData[index2] == 2 | self.MouseData[index2] == 1 && self.viewMode != self.MouseData[index2])
          {
            index1 = index2;
            break;
          }
        }
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut num: i32 = DrawMod.RandyNumber.Next(0, 1000);
        if (num <= 600 & self.Info1Id > 0)
        {
          let mut index3: i32 = self.SubpartNr(self.Info1Id);
          if (index3 > -1)
          {
            x = self.SubPartX[index3] + 3;
            y = self.SubPartY[index3] + 3;
          }
        }
        else if (num <= 130 & index1 > -1)
        {
          x = self.MouseRect[index1].X + 3;
          y = self.MouseRect[index1].Y + 3;
        }
        else if (num <= 150 & self.sizeId > 0)
        {
          let mut index4: i32 = self.SubpartNr(self.sizeId);
          if (index4 > -1)
          {
            x = self.SubPartX[index4] + 3;
            y = self.SubPartY[index4] + 3;
          }
        }
        else
        {
          x = DrawMod.RandyNumber.Next(60, self.w - 420);
          y = DrawMod.RandyNumber.Next(30, self.h - 50);
        }
        if (x > 0)
        {
          windowReturnClass2: WindowReturnClass = self.HandleMouseClick(x, y, 1);
          if (windowReturnClass2.Counter > -1)
            ;
          return windowReturnClass2;
        }
      }
      return windowReturnClass1;
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = base.HandleMouseMove(x, y);
      bool flag = false;
      let mut num1: i32 = -1;
      let mut num2: i32 = -1;
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          if (self.cardSize == 1 && self.MouseData[mouseCounter] >= 100 & self.MouseData[mouseCounter] <= 100 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter + 100)
          {
            if (self.hovernr == self.MouseData[mouseCounter] - 100)
            {
              flag = true;
              break;
            }
            flag = true;
            self.hovernr = self.MouseData[mouseCounter] - 100;
            self.dostuff();
            windowReturnClass2.SetFlag(true);
            break;
          }
          if (self.MouseData[mouseCounter] >= 1 & self.MouseData[mouseCounter] <= 2)
            num1 = self.MouseData[mouseCounter];
          if (self.MouseData[mouseCounter] >= 50100 & self.MouseData[mouseCounter] <= 50100 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter + 100)
            num2 = self.MouseData[mouseCounter] - 50100;
        }
      }
      if (num1 > 0)
      {
        if (self.mouseOverWhichTab != num1)
        {
          if (self.game.EmpireStyle)
            SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
          self.mouseOverWhichTab = num1;
          self.dostuff();
          windowReturnClass2.SetFlag(true);
          return windowReturnClass2;
        }
      }
      else
      {
        if (self.mouseOverWhichTab > 0)
        {
          self.mouseOverWhichTab = -1;
          self.dostuff();
          windowReturnClass2.SetFlag(true);
          return windowReturnClass2;
        }
        if (num2 >= 0)
        {
          if (self.scrapMouseOver != num2)
          {
            if (self.game.EmpireStyle)
              SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
            self.scrapMouseOver = num2;
            self.dostuff();
            windowReturnClass2.SetFlag(true);
            return windowReturnClass2;
          }
        }
        else if (num2 == -1 & self.scrapMouseOver != -1)
        {
          if (self.game.EmpireStyle)
            SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
          self.scrapMouseOver = -1;
          self.dostuff();
          windowReturnClass2.SetFlag(true);
          return windowReturnClass2;
        }
      }
      if (!flag & self.hovernr > -1)
      {
        self.hovernr = -1;
        self.dostuff();
        windowReturnClass2.SetFlag(true);
      }
      return windowReturnClass2;
    }

    pub fn Before_DoRefresh_Called_By_FlagAllIncludingRefresh()
    {
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
      if (self.game.EditObj.se1_CardsCategory > 0)
      {
        if (self.categorySelectMode[self.currentCat] == 4)
        {
          self.miniSelectValue =  Math.Round(Conversion.Val( self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0))].FindRow(0, self.miniSelectLeader)));
          if (self.miniSelectValue < 0)
            self.miniSelectLeader = -1;
        }
        if (self.categorySelectMode[self.currentCat] == 3 & self.miniUnitSelect > -1 && self.miniUnitSelect > self.game.Data.UnitCounter | self.miniUnitSelect < 1)
          self.miniUnitSelect = -1;
        if (self.categorySelectMode[self.currentCat] == 5 & self.miniUnitSelect > -1 && self.miniUnitSelect > self.game.Data.UnitCounter | self.miniUnitSelect < 1)
          self.miniUnitSelect = -1;
        if (self.categorySelectMode[self.currentCat] == 1 & self.miniSelectValue > -1)
        {
          self.game.EditObj.se1_CardsSelectX = self.miniSelectX;
          self.game.EditObj.se1_CardsSelectY = self.miniSelectY;
          if (self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0))].FindRow(0, self.miniSelectValue) < 0)
            self.miniSelectValue = -1;
          bool flag = false;
          if (self.game.EditObj.se1_CardsSelectX > -1 & self.game.EditObj.se1_CardsSelectY > -1 & self.game.EditObj.se1_CardsSelectX <= self.game.Data.MapObj[0].MapWidth & self.game.EditObj.se1_CardsSelectY <= self.game.Data.MapObj[0].MapHeight)
          {
            if (self.game.Data.MapObj[0].HexObj[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY].Regime > -1)
            {
              if (self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY].Regime].id != self.miniSelectValue)
                flag = true;
            }
            else
              flag = true;
          }
          if (self.miniSelectValue > 0 & flag)
          {
            let mut num1: i32 = -1;
            let mut num2: i32 = -1;
            let mut num3: i32 = 0;
            do
            {
              let mut num4: i32 = mapWidth;
              for (let mut index1: i32 = 0; index1 <= num4; index1 += 1)
              {
                let mut num5: i32 = mapHeight;
                for (let mut index2: i32 = 0; index2 <= num5; index2 += 1)
                {
                  if (self.game.Data.MapObj[0].HexObj[index1, index2].Regime > -1 && num3 == 1 | self.game.Data.MapObj[0].HexObj[index1, index2].MaxRecon > 0 && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index1, index2].Regime].id == self.miniSelectValue & num1 == -1)
                  {
                    num1 = index1;
                    num2 = index2;
                  }
                }
              }
              num3 += 1;
            }
            while (num3 <= 1);
            if (num1 > -1)
            {
              self.game.EditObj.se1_CardsSelectX = num1;
              self.game.EditObj.se1_CardsSelectY = num2;
            }
            else
              self.miniSelectValue = -1;
          }
        }
        if (self.categorySelectMode[self.currentCat] == 2 & self.miniSelectValue > -1)
        {
          self.game.EditObj.se1_CardsSelectX = self.miniSelectX;
          self.game.EditObj.se1_CardsSelectY = self.miniSelectY;
          if (self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0))].FindRow(0, self.miniSelectValue) < 0)
            self.miniSelectValue = -1;
          bool flag = false;
          if (self.game.EditObj.se1_CardsSelectX > -1 & self.game.EditObj.se1_CardsSelectY > -1 && self.zones.Value[self.game.EditObj.se1_CardsSelectX, self.game.EditObj.se1_CardsSelectY] != self.miniSelectValue)
            flag = true;
          if (self.miniSelectValue > 0 & flag)
          {
            let mut num6: i32 = -1;
            let mut num7: i32 = -1;
            let mut num8: i32 = 0;
            do
            {
              let mut num9: i32 = mapWidth;
              for (let mut index3: i32 = 0; index3 <= num9; index3 += 1)
              {
                let mut num10: i32 = mapHeight;
                for (let mut index4: i32 = 0; index4 <= num10; index4 += 1)
                {
                  if (num8 == 1 | self.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0 && self.zones.Value[index3, index4] == self.miniSelectValue & num6 == -1)
                  {
                    num6 = index3;
                    num7 = index4;
                  }
                }
              }
              num8 += 1;
            }
            while (num8 <= 1);
            if (num6 > -1)
            {
              self.game.EditObj.se1_CardsSelectX = num6;
              self.game.EditObj.se1_CardsSelectY = num7;
            }
            else
              self.miniSelectValue = -1;
          }
        }
        self.miniSelectX = self.game.EditObj.se1_CardsSelectX;
        self.miniSelectY = self.game.EditObj.se1_CardsSelectY;
      }
      if (self.miniSelectX < 0 | self.miniSelectY < 0)
      {
        self.miniSelectX = self.game.SelectX;
        self.miniSelectY = self.game.SelectY;
      }
      self.prepareTempValue4();
      self.donePrepareCardPlayable = false;
      self.dostuff();
    }

    pub fn PopUpRefresh()
    {
      self.DoRefresh();
      self.prepareTempValue4();
      self.donePrepareCardPlayable = false;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          if (self.MouseData[mouseCounter] >= 1 & self.MouseData[mouseCounter] <= 2)
          {
            self.viewMode = self.MouseData[mouseCounter];
            if (self.OptionsListId > 0)
            {
              self.RemoveSubPart(self.OptionsListId);
              self.OptionsListId = 0;
            }
            if (self.OptionsList2Id > 0)
            {
              self.RemoveSubPart(self.OptionsList2Id);
              self.OptionsList2Id = 0;
            }
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] == 9999999)
          {
            self.game.EditObj.SetViewMode2 = 0;
            if (self.game.EditObj.GuiDown)
            {
              self.game.EditObj.GuiDown = false;
              self.game.EditObj.SetViewMode2 = 0;
              windowReturnClass.AddCommand(3, 11);
            }
            else
            {
              windowReturnClass.AddCommand(1, 9);
              windowReturnClass.AddCommand(7, 12);
              windowReturnClass.AddCommand(4, 67);
            }
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] == 11)
          {
            self.game.EditObj.UDSpopupText = "";
            self.formref.Cursor = Cursors.WaitCursor;
            self.game.EditObj.UDSClearInput();
            self.game.EventRelatedObj.SetUDSKey("CHARID", self.MouseData2[mouseCounter]);
            self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 531, 0, 0));
            self.formref.Cursor = Cursors.Default;
            self.game.EditObj.PopupValue = 21;
            windowReturnClass.AddCommand(5, 14);
            self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] >= 100 & self.MouseData[mouseCounter] <= 100 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter + 100)
          {
            if (self.MouseData[mouseCounter] - 100 != self.detailnr)
            {
              self.detailnr = self.MouseData[mouseCounter] - 100;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          else
          {
            if (self.MouseData[mouseCounter] >= 50100 & self.MouseData[mouseCounter] <= 100 + self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter + 50100)
            {
              if (self.game.EmpireStyle)
                SoundMod.PlayAWave(self.game.AppPath + "sound/specials/cash.wav",  self.game.EditObj);
              let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
              let mut setValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPoints", 2))) + self.MouseData2[mouseCounter];
              self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPoints", 2, setValue, true);
              let mut num: i32 = self.MouseData[mouseCounter] - 50100;
              for (let mut actionCardCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter; actionCardCounter >= 0; actionCardCounter += -1)
              {
                if (actionCardCounter == num)
                {
                  let mut nr: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[actionCardCounter];
                  self.game.Data.RegimeObj[self.game.Data.Turn].RemoveActionCard(actionCardCounter);
                  self.game.Data.RemoveActionCard(nr);
                  break;
                }
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (self.MouseData[mouseCounter] > 100000)
            {
              self.detailnr = -1;
              self.lastActualCard = -1;
              self.pageNr = 0;
              self.currentCat = self.MouseData[mouseCounter] - 100000;
              self.miniCatSelectValue = -1;
              self.miniUnitSelect = -1;
              self.miniSelectValue = -1;
              self.miniSelectLeader = -1;
              self.prepareTempValue4();
              self.donePrepareCardPlayable = false;
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          if (self.MouseData[mouseCounter] == 51)
          {
            let mut index1: i32 = self.SubpartNr(self.miniId);
            Coordinate hexCoord = ((MiniMapPartClass) self.SubPartList[index1]).GetHexCoord(x - self.SubPartX[index1], y - self.SubPartY[index1]);
            if (hexCoord.onmap)
            {
              if (self.categorySelectMode[self.currentCat] == 1)
              {
                let mut index2: i32 = self.game.EditObj.TempAI[hexCoord.x, hexCoord.y];
                if (index2 > -1)
                {
                  let mut id: i32 = self.game.Data.RegimeObj[index2].id;
                  if (self.miniSelectValue != id)
                  {
                    self.miniSelectValue = id;
                    self.miniSelectX = hexCoord.x;
                    self.miniSelectY = hexCoord.y;
                    self.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
              }
              if (self.categorySelectMode[self.currentCat] == 2)
              {
                let mut index3: i32 = self.game.EditObj.TempValue4[0].Value[hexCoord.x, hexCoord.y];
                if (index3 > 0 && self.zoneVisible[index3] && self.miniSelectValue != index3)
                {
                  self.miniSelectValue = index3;
                  self.miniSelectX = hexCoord.x;
                  self.miniSelectY = hexCoord.y;
                  self.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
          }
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index4: i32 = 0; index4 <= subPartCounter; index4 += 1)
        {
          if (x > self.SubPartX[index4] & x < self.SubPartX[index4] + self.SubPartW[index4] && y > self.SubPartY[index4] & y < self.SubPartY[index4] + self.SubPartH[index4])
          {
            let mut num1: i32 = self.SubPartID[index4];
            if (num1 == self.nextId)
            {
              self += 1.pageNr;
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.prevId)
            {
              --self.pageNr;
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.sizeId)
            {
              self.cardSize = self.cardSize != 1 ? 1 : 2;
              self.pageNr = 1;
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.scrapId)
            {
              self.scrapMode = !self.scrapMode;
              if (!self.scrapMode)
                self.donePrepareCardPlayable = false;
              if (self.OptionsListId > 0)
              {
                self.RemoveSubPart(self.OptionsListId);
                self.OptionsListId = 0;
              }
              if (self.OptionsList2Id > 0)
              {
                self.RemoveSubPart(self.OptionsList2Id);
                self.OptionsList2Id = 0;
              }
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut index5: i32 = self.SubPartList[index4].Click(x - self.SubPartX[index4], y - self.SubPartY[index4]);
              if (index5 > -1)
              {
                self.SubPartFlag[index4] = true;
                if (self.categorySelectMode[self.currentCat] == 1)
                {
                  if (self.miniSelectValue == self.game.Data.RegimeObj[index5].id)
                    ;
                  self.miniSelectValue = self.game.Data.RegimeObj[index5].id;
                  Coordinate coordinate;
                  coordinate.onmap = false;
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index6: i32 = 0; index6 <= mapWidth; index6 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index7: i32 = 0; index7 <= mapHeight; index7 += 1)
                    {
                      if (self.game.Data.MapObj[0].HexObj[index6, index7].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index6, index7].Regime].id == self.miniSelectValue && self.game.Data.MapObj[0].HexObj[index6, index7].MaxRecon > 0)
                      {
                        coordinate.x = index6;
                        coordinate.y = index7;
                        coordinate.onmap = true;
                        break;
                      }
                    }
                    if (coordinate.onmap)
                      break;
                  }
                  if (coordinate.onmap)
                  {
                    self.miniSelectX = coordinate.x;
                    self.miniSelectY = coordinate.y;
                  }
                }
                if (self.categorySelectMode[self.currentCat] == 2)
                {
                  if (self.miniSelectValue == index5)
                    ;
                  self.miniSelectValue = index5;
                  Coordinate coordinate;
                  coordinate.onmap = false;
                  let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index8: i32 = 0; index8 <= mapWidth; index8 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index9: i32 = 0; index9 <= mapHeight; index9 += 1)
                    {
                      if (self.zones.Value[index8, index9] > 0 && self.game.Data.MapObj[0].HexObj[index8, index9].MaxRecon > 0 && self.zones.Value[index8, index9] == self.miniSelectValue)
                      {
                        coordinate.x = index8;
                        coordinate.y = index9;
                        coordinate.onmap = true;
                        break;
                      }
                    }
                    if (coordinate.onmap)
                      break;
                  }
                  if (coordinate.onmap)
                  {
                    self.miniSelectX = coordinate.x;
                    self.miniSelectY = coordinate.y;
                  }
                }
                if (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5)
                {
                  self.miniUnitSelect = index5;
                  Coordinate coordinate;
                  coordinate.onmap = false;
                }
                if (self.categorySelectMode[self.currentCat] == 4)
                {
                  let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
                  self.miniSelectValue = index5;
                  self.miniSelectLeader =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.miniSelectValue, 0]));
                  Coordinate coordinate;
                  coordinate.onmap = false;
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else if (num1 == self.OptionsList2Id)
            {
              let mut num2: i32 = self.SubPartList[index4].Click(x - self.SubPartX[index4], y - self.SubPartY[index4]);
              if (num2 > -1)
              {
                self.SubPartFlag[index4] = true;
                if (self.categorySelectMode[self.currentCat] == 3 | self.categorySelectMode[self.currentCat] == 5)
                {
                  self.miniUnitSelect = -1;
                  self.miniCatSelectValue = num2;
                  Coordinate coordinate;
                  coordinate.onmap = false;
                }
                if (self.categorySelectMode[self.currentCat] == 4)
                {
                  self.miniSelectValue = -1;
                  self.miniCatSelectValue = num2;
                  self.miniSelectLeader = -1;
                  Coordinate coordinate;
                  coordinate.onmap = false;
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else if (num1 == self.pageId)
            {
              let mut enr: i32 = self.SubPartList[index4].Click(x - self.SubPartX[index4], y - self.SubPartY[index4]);
              if (enr > 0)
              {
                self.game.EditObj.UDSpopupText = "";
                self.formref.Cursor = Cursors.WaitCursor;
                self.game.EventRelatedObj.DoCheckSpecificEvent(enr);
                self.formref.Cursor = Cursors.Default;
                if (self.game.EditObj.UDSpopupText.Length > 1)
                {
                  self.game.EditObj.PopupValue = 21;
                  self.game.EditObj.udsLastCalledPopupEventNr = enr;
                  windowReturnClass.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.SubPartFlag[index4] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.buyScrapId)
              {
                if (self.game.EmpireStyle)
                  SoundMod.PlayAWave(self.game.AppPath + "sound/specials/powerup.wav",  self.game.EditObj);
                self.game.EventRelatedObj.ExecMakeAndGiveScrapCard(self.game.Data.Turn);
                let mut num3: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCardCounter];
                str: String = "New Stratagem has been crafted!";
                let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
                let mut num4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPoints", 2)));
                let mut num5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPointCost", 2)));
                if (num5 < 15)
                  num5 = 15;
                let mut setValue1: i32 = num4 - num5;
                if (0 > setValue1)
                  setValue1 = 0;
                let mut setValue2: i32 = num5 + Math.Max(1,  Math.Round(Math.Floor( num5 / 10.0)));
                self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPoints", 2, setValue1, true);
                self.game.Data.StringListObj[stringListById].SetData2(0, self.game.Data.RegimeObj[self.game.Data.Turn].id, 1, "scrapPointCost", 2, setValue2, true);
                self.game.EditObj.PopupValue = 29;
                self.game.EditObj.QuestionText = str;
                self.game.EditObj.QuestionCard = num3;
                self.game.EditObj.AnswerCount = 1;
                self.game.EditObj.AnswerText[1] = "Okay";
                self.game.EditObj.AnswerTextMouseOver[1] = "Acknowledge the receival of this new Scrap Stragem.";
                windowReturnClass.AddCommand(5, 14);
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                self.scrapMode = false;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.Info1Id)
              {
                self.game.EditObj.se1_map_data3cache_set = false;
                let mut unitSelected: i32 = self.game.EditObj.UnitSelected;
                self.lastActualCard = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr];
                let mut cardNr: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr];
                self.game.EditObj.SkippedPreSelectPopup = false;
                self.viewMode = 1;
                if (self.OptionsListId > 0)
                {
                  self.RemoveSubPart(self.OptionsListId);
                  self.OptionsListId = 0;
                }
                if (self.OptionsList2Id > 0)
                {
                  self.RemoveSubPart(self.OptionsList2Id);
                  self.OptionsList2Id = 0;
                }
                if (self.game.Data.ActionCardObj[cardNr].AreaSlot > -1 & self.categorySelectMode[self.currentCat] < 1)
                {
                  if (self.game.EditObj.AreaX == -1)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj.DoCardSlot = self.detailnr;
                    self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, self.detailnr);
                    if (self.game.HandyFunctionsObj.CardSelectHexTestPossible(cardNr, self.game.Data.ActionCardObj[cardNr].AreaSlot, self.game.Data.ActionCardObj[cardNr].AreaValue))
                    {
                      self.game.FormRef.Cursor = Cursors.Default;
                      self.game.EditObj.DoCardSlot = self.detailnr;
                      self.game.EditObj.HandCard = -1;
                      self.game.EditObj.AreaSlot = -1;
                      self.game.EditObj.AreaValue = -1;
                      self.game.EditObj.AreaX = -1;
                      self.game.EditObj.AreaY = -1;
                      self.game.EditObj.AreaMap = -1;
                      self.game.EditObj.AreaSlot = self.game.Data.ActionCardObj[cardNr].AreaSlot;
                      self.game.EditObj.AreaValue = self.game.Data.ActionCardObj[cardNr].AreaValue;
                      self.game.EditObj.PopupValue = 1;
                      windowReturnClass.AddCommand(5, 14);
                      self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    self.game.EditObj.SkippedPreSelectPopup = true;
                  }
                  else
                  {
                    let mut num6: i32 =  Interaction.MsgBox( "Error. Cant have selected an Area X,Y already.");
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (self.game.Data.ActionCardObj[cardNr].UnitSelect & self.categorySelectMode[self.currentCat] < 1)
                {
                  self.game.ProcessingObj.PlayCardPreEvent(self.game.Data.Turn, self.detailnr);
                  if (self.game.HandyFunctionsObj.CardSelectUnitTestPossible(cardNr))
                  {
                    self.game.EditObj.DoCardSlot = self.detailnr;
                    self.game.EditObj.HandCard = -1;
                    self.game.EditObj.AreaSlot = -1;
                    self.game.EditObj.AreaValue = -1;
                    self.game.EditObj.AreaX = -1;
                    self.game.EditObj.AreaY = -1;
                    self.game.EditObj.AreaMap = -1;
                    self.game.EditObj.PopupValue = 3;
                    self.game.EditObj.UnitSelected = self.miniUnitSelect;
                    windowReturnClass.AddCommand(5, 14);
                    self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  self.game.EditObj.SkippedPreSelectPopup = true;
                }
                let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 277, 0, 0));
                bool dontDelete = false;
                if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, self.game.Data.ActionCardObj[self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.detailnr]].TempVar0, 16))) > 0)
                  dontDelete = true;
                if ( self.game.Data.RuleVar[408] > 0.0)
                {
                  let mut selectX: i32 = self.game.SelectX;
                  let mut selectY: i32 = self.game.SelectY;
                  self.game.SelectX = self.miniSelectX;
                  self.game.SelectY = self.miniSelectY;
                  self.game.EditObj.DoCardSlot = self.detailnr;
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  if (self.categorySelectMode[self.currentCat] == 1)
                  {
                    let mut regime: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime;
                    let mut num7: i32 = 0;
                    if (regime > -1)
                    {
                      if (self.game.Data.RegimeObj[regime].id != self.miniSelectValue)
                        num7 = 1;
                    }
                    else
                      num7 = 1;
                    if (num7 == 1)
                    {
                      let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
                      for (let mut index10: i32 = 0; index10 <= mapWidth1; index10 += 1)
                      {
                        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                        for (let mut index11: i32 = 0; index11 <= mapHeight; index11 += 1)
                        {
                          if (self.game.Data.MapObj[0].HexObj[index10, index11].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index10, index11].Regime].id == self.miniSelectValue && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index10, index11].Regime].id == self.game.EditObj.TempValue4[0].Value[self.game.SelectX, self.game.SelectY])
                          {
                            self.game.SelectX = index10;
                            self.game.SelectY = index11;
                            num7 = 0;
                            break;
                          }
                        }
                      }
                      if (num7 == 1)
                      {
                        let mut mapWidth2: i32 = self.game.Data.MapObj[0].MapWidth;
                        for (let mut index12: i32 = 0; index12 <= mapWidth2; index12 += 1)
                        {
                          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                          for (let mut index13: i32 = 0; index13 <= mapHeight; index13 += 1)
                          {
                            if (self.game.Data.MapObj[0].HexObj[index12, index13].Regime > -1 && self.game.Data.RegimeObj[self.game.Data.MapObj[0].HexObj[index12, index13].Regime].id == self.miniSelectValue)
                            {
                              self.game.SelectX = index12;
                              self.game.SelectY = index13;
                              break;
                            }
                          }
                        }
                      }
                    }
                  }
                  if (self.categorySelectMode[self.currentCat] == 2 && self.zones.Value[self.game.SelectX, self.game.SelectY] != self.miniSelectValue)
                  {
                    let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                    for (let mut index14: i32 = 0; index14 <= mapWidth; index14 += 1)
                    {
                      let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                      for (let mut index15: i32 = 0; index15 <= mapHeight; index15 += 1)
                      {
                        if (self.game.Data.MapObj[0].HexObj[index14, index15].Regime > -1 && self.zones.Value[index14, index15] == self.miniSelectValue && self.zones.Value[index14, index15] == self.game.EditObj.TempValue4[0].Value[self.game.SelectX, self.game.SelectY])
                        {
                          self.game.SelectX = index14;
                          self.game.SelectY = index15;
                        }
                      }
                    }
                  }
                  self.game.ProcessingObj.PlayCard(self.game.Data.Turn, self.detailnr, dontDelete);
                  if (self.miniUnitSelect > 0 & self.categorySelectMode[self.currentCat] == 3)
                  {
                    let mut num8: i32 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.miniUnitSelect].Historical].GiveHisVarValue(28);
                    if (num8 != 0)
                      self.game.Data.UnitObj[self.miniUnitSelect].SODefendPercent = 100 - num8;
                    else
                      self.game.Data.UnitObj[self.miniUnitSelect].SODefendPercent = 50;
                  }
                  self.game.EditObj.DoCardSlot = -1;
                  windowReturnClass.SetFlag(true);
                  self.game.EditObj.PopupValue = 21;
                  windowReturnClass.AddCommand(5, 14);
                  windowReturnClass.SetFlag(true);
                  self.game.SelectX = selectX;
                  self.game.SelectY = selectY;
                  return windowReturnClass;
                }
                self.game.EditObj.UnitSelected = unitSelected;
                let mut messCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
                self.game.ProcessingObj.PlayCard(self.game.Data.Turn, self.detailnr, dontDelete);
                self.game.EditObj.HandCard = -1;
                self.game.EditObj.DoCardSlot = -1;
                if (self.game.EditObj.DoQuit)
                {
                  self.game.Data = DataClass::new();
                  self.game.EditObj.DoQuit = false;
                  self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                  if (self.game.Data.UseAI == 1)
                    self.game.NewAIObj.LastRegime = -1;
                  self.game.EditObj.ShowInitialMenu = true;
                  windowReturnClass.AddCommand(3, 12);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (Strings.Len(self.game.Data.LoadGame) > 0 & self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter <= messCounter)
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  Form formRef =  self.game.FormRef;
                  self.game.HandyFunctionsObj.LoadGameNow();
                  self.game.FormRef = (Form1) formRef;
                  self.game.FormRef.Cursor = Cursors.Default;
                  windowReturnClass.AddCommand(3, 13);
                  return windowReturnClass;
                }
                if (self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter > messCounter)
                {
                  self.game.EditObj.AreaSlot = -1;
                  self.game.EditObj.AreaValue = -1;
                  self.game.EditObj.AreaX = -1;
                  self.game.EditObj.AreaY = -1;
                  self.game.EditObj.AreaMap = -1;
                  self.game.EditObj.PopupValue = 0;
                  self.game.EditObj.FromMessage = messCounter + 1;
                  windowReturnClass.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.DoRefresh();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            return windowReturnClass;
          }
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub Rectangle DrawOneTab(
      Graphics g,
      bool wideTab,
      bool active,
      tx: i32,
      ty: i32,
      sHeader: String,
      sText: String,
      spriteSlot: i32,
      iconSlot: i32,
      let mut smallNumber: i32 = -1,
      bool grayedOut = false,
      let mut textOffsetX: i32 = 0,
      let mut spriteOffsetY: i32 = 0,
      bool tMousingOverNow = false)
    {
      bitmap1: Bitmap;
      if (tMousingOverNow)
      {
        if (active & wideTab)
        {
           let mut local1: &Graphics = &g;
          bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1HIGH);
           let mut local2: &Bitmap = &bitmap2;
          let mut x: i32 = tx;
          let mut y: i32 = ty;
          DrawMod.Draw( local1,  local2, x, y, 0.05f, 0.05f, 0.05f, 1f);
        }
        if (!active & wideTab)
        {
           let mut local3: &Graphics = &g;
          bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1LOW);
           let mut local4: &Bitmap = &bitmap3;
          let mut x: i32 = tx;
          let mut y: i32 = ty;
          DrawMod.Draw( local3,  local4, x, y, 0.05f, 0.05f, 0.05f, 1f);
        }
      }
      else
      {
        if (active & wideTab)
        {
           let mut local5: &Graphics = &g;
          bitmap1 = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1HIGH);
           let mut local6: &Bitmap = &bitmap1;
          let mut x: i32 = tx;
          let mut y: i32 = ty;
          DrawMod.DrawSimple( local5,  local6, x, y);
        }
        if (!active & wideTab)
        {
           let mut local7: &Graphics = &g;
          bitmap1 = BitmapStore.GetBitmap(self.game.SE1_ORDERBAR_TAB1LOW);
           let mut local8: &Bitmap = &bitmap1;
          let mut x: i32 = tx;
          let mut y: i32 = ty;
          DrawMod.DrawSimple( local7,  local8, x, y);
        }
      }
      if (wideTab)
      {
        if (spriteSlot > 0)
        {
          if (active)
          {
             let mut local9: &Graphics = &g;
            bitmap1 = BitmapStore.GetBitmap(spriteSlot);
             let mut local10: &Bitmap = &bitmap1;
            let mut x: i32 = tx + 3;
            let mut y: i32 = ty + 4 + spriteOffsetY;
            DrawMod.DrawSimple( local9,  local10, x, y);
          }
          if (!active)
          {
             let mut local11: &Graphics = &g;
            bitmap1 = BitmapStore.GetBitmap(spriteSlot);
             let mut local12: &Bitmap = &bitmap1;
            let mut x: i32 = tx + 3;
            let mut y: i32 = ty + 11 + spriteOffsetY;
            DrawMod.DrawSimple( local11,  local12, x, y);
          }
        }
        else if (iconSlot > -1 && !grayedOut)
        {
          Rectangle rectangle1;
          Rectangle rectangle2;
          if (active)
          {
             let mut local13: &Graphics = &g;
            bitmap1 = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local14: &Bitmap = &bitmap1;
            rectangle1 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(tx + 4, ty + 11, 42, 32);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
          }
          if (!active)
          {
             let mut local15: &Graphics = &g;
            bitmap1 = BitmapStore.GetBitmap(self.game.SE1_ICONS);
             let mut local16: &Bitmap = &bitmap1;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 4, ty + 18, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
          }
        }
      }
      g.MeasureString(sText, DrawMod.TGame.MarcFont16);
      c: Color;
      color: Color;
      if (active)
      {
        c = self.game.seColWhite;
        color = self.game.seColGray;
      }
      if (!active)
      {
        c = self.game.seColGray;
        color = self.game.seColGray;
      }
      if (grayedOut)
      {
        c = Color.FromArgb( byte.MaxValue, 128, 128, 128);
        color = Color.FromArgb( byte.MaxValue, 128, 128, 128);
      }
      if (active)
      {
        if (wideTab)
          DrawMod.DrawTextColouredConsoleCenter( g, sText, self.game.MarcFont16, tx + 97, ty + 18, c);
      }
      else if (wideTab)
        DrawMod.DrawTextColouredConsoleCenter( g, sText, self.game.MarcFont16, tx + 97, ty + 25, c);
      Rectangle rectangle = Rectangle::new(tx, ty, 200, 50);
      if (!wideTab)
        rectangle = Rectangle::new(tx, ty, 75, 50);
      return rectangle;
    }
  }
}
