// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFDesignWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SFDesignWindowClass : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     b7bid: i32;
     B7TextId: i32;
     B8Id: i32;
     b8bid: i32;
     B8TextId: i32;
     B9Id: i32;
     b9bid: i32;
     B9TextId: i32;
     B10Id: i32;
     B10TextId: i32;
     B11Id: i32;
     B11TextId: i32;
     off1id: i32;
     detailnr: i32;
     optnr: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Pic1Id: i32;
     detailnr2: i32;
     OrderTextId: i32;
     OrderText2Id: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     ExtraId: i32;
     steppy: i32;
     typpy: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     bool Hq;

    pub fn DoRefresh()
    {
      self.game.HandyFunctionsObj.CalculateSFDesignCosts();
      self.dostuff();
    }

    pub SFDesignWindowClass(
       tGame: GameClass,
      screenbitmap: Bitmap = null,
      let mut sx: i32 = -1,
      let mut sy: i32 = -1,
      let mut tl1: i32 = -1,
      let mut tl2: i32 = -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      self.steppy = 0;
      self.typpy = -1;
      self.detailnr = -1;
      self.detailnr2 = -1;
      self.optnr = 1;
      if (tl1 > -1)
      {
        self.optnr = tl1;
        self.detailnr = tl2;
      }
      self.game.HandyFunctionsObj.CalculateSFDesignCosts();
      self.dostuff();
    }

     void dostuff()
    {
      if (self.off1id > 0)
        self.RemoveSubPart(self.off1id);
      if (self.Text1Id > 0)
        self.RemoveSubPart(self.Text1Id);
      if (self.Text2Id > 0)
        self.RemoveSubPart(self.Text2Id);
      if (self.Text3Id > 0)
        self.RemoveSubPart(self.Text3Id);
      if (self.Pic1Id > 0)
        self.RemoveSubPart(self.Pic1Id);
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B8Id > 0)
        self.RemoveSubPart(self.B8Id);
      if (self.B9Id > 0)
        self.RemoveSubPart(self.B9Id);
      if (self.b7bid > 0)
        self.RemoveSubPart(self.b7bid);
      if (self.b8bid > 0)
        self.RemoveSubPart(self.b8bid);
      if (self.b9bid > 0)
        self.RemoveSubPart(self.b9bid);
      if (self.B10Id > 0)
        self.RemoveSubPart(self.B10Id);
      if (self.B11Id > 0)
        self.RemoveSubPart(self.B11Id);
      if (self.OrderUpId > 0)
        self.RemoveSubPart(self.OrderUpId);
      if (self.OrderDownId > 0)
        self.RemoveSubPart(self.OrderDownId);
      if (self.ExtraId > 0)
        self.RemoveSubPart(self.ExtraId);
      if (self.OrderTextId > 0)
        self.RemoveSubPart(self.OrderTextId);
      if (self.OrderText2Id > 0)
        self.RemoveSubPart(self.OrderText2Id);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      if (self.OptionsList2Id > 0)
        self.RemoveSubPart(self.OptionsList2Id);
      self.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.optnr == 1)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONFLAGGED);
        self.B1Id = self.AddSubPart( tsubpart, 5, 10, 32, 32, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED);
        self.B1Id = self.AddSubPart( tsubpart, 5, 10, 32, 32, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("All", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 24, false);
      self.B1TextId = self.AddSubPart( tsubpart1, 40, 9, 60, 24, 0);
      if (self.optnr == 2)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONFLAGGED);
        self.B2Id = self.AddSubPart( tsubpart1, 5, 40, 32, 32, 1);
      }
      else
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED);
        self.B2Id = self.AddSubPart( tsubpart1, 5, 40, 32, 32, 1);
      }
      tsubpart1 =  TextPartClass::new("New", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 24, false);
      self.B2TextId = self.AddSubPart( tsubpart1, 40, 39, 60, 24, 0);
      if (self.optnr == 3)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONFLAGGED);
        self.B3Id = self.AddSubPart( tsubpart1, 5, 70, 32, 32, 1);
      }
      else
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED);
        self.B3Id = self.AddSubPart( tsubpart1, 5, 70, 32, 32, 1);
      }
      tsubpart1 =  TextPartClass::new("Upgrade", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 24, false);
      self.B3TextId = self.AddSubPart( tsubpart1, 40, 69, 60, 24, 0);
      if (self.optnr == 4)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONFLAGGED);
        self.B4Id = self.AddSubPart( tsubpart1, 5, 100, 32, 32, 1);
      }
      else
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONUNFLAGGED);
        self.B4Id = self.AddSubPart( tsubpart1, 5, 100, 32, 32, 1);
      }
      tsubpart1 =  TextPartClass::new("Improve", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 24, false);
      self.B4TextId = self.AddSubPart( tsubpart1, 40, 99, 60, 24, 0);
      self.OptionsListObj = ListClass::new();
      let mut num1: i32 = -1;
      let mut num2: i32 = -1;
      let mut sfTypeCounter1: i32 = self.game.Data.SFTypeCounter;
      for (let mut tdata: i32 = 0; tdata <= sfTypeCounter1; tdata += 1)
      {
        let mut num3: i32 = 1;
        let mut sfTypeCounter2: i32 = self.game.Data.SFTypeCounter;
        for (let mut index: i32 = 0; index <= sfTypeCounter2; index += 1)
        {
          if (self.game.Data.SFTypeObj[index].ModelID == self.game.Data.SFTypeObj[tdata].ModelID && self.game.Data.SFTypeObj[tdata].ModelBaseModel == self.game.Data.SFTypeObj[index].ModelBaseModel && self.game.Data.SFTypeObj[tdata].ModelMark == self.game.Data.SFTypeObj[index].ModelMark && self.game.Data.SFTypeObj[tdata].ModelVersion < self.game.Data.SFTypeObj[index].ModelVersion)
            num3 = 0;
        }
        if (num3 == 1)
        {
          if ((self.optnr == 1 | self.optnr == 2) & self.game.Data.SFTypeObj[tdata].TempNewCost > -1 & (self.game.Data.SFTypeObj[tdata].ModelRegime == self.game.Data.Turn | self.game.Data.SFTypeObj[tdata].ModelRegime == -1))
          {
            num2 += 1;
            self.OptionsListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
            if (self.detailnr == tdata)
              num1 = num2;
          }
          else if (self.optnr == 1 & self.game.Data.SFTypeObj[tdata].ModelID > 0 & self.game.Data.SFTypeObj[tdata].ModelRegime == self.game.Data.Turn)
          {
            num2 += 1;
            self.OptionsListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
            if (self.detailnr == tdata)
              num1 = num2;
          }
          else if (self.optnr == 1 & self.game.Data.SFTypeObj[tdata].TempUpgradeCost > -1 & self.game.Data.SFTypeObj[tdata].ModelRegime == self.game.Data.Turn)
          {
            num2 += 1;
            self.OptionsListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
            if (self.detailnr == tdata)
              num1 = num2;
          }
          else if (self.optnr == 3 & self.game.Data.SFTypeObj[tdata].TempUpgradeCost > -1 & self.game.Data.SFTypeObj[tdata].ModelRegime == self.game.Data.Turn)
          {
            let mut num4: i32 = 1;
            let mut sfTypeCounter3: i32 = self.game.Data.SFTypeCounter;
            for (let mut index: i32 = 0; index <= sfTypeCounter3; index += 1)
            {
              if (self.game.Data.SFTypeObj[index].ModelID == self.game.Data.SFTypeObj[tdata].ModelID & self.game.Data.SFTypeObj[index].ModelVersion > self.game.Data.SFTypeObj[tdata].ModelVersion)
                num4 = 0;
            }
            if (num4 == 1)
            {
              num2 += 1;
              self.OptionsListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
              if (self.detailnr == tdata)
                num1 = num2;
            }
          }
          else if (self.optnr == 4 & self.game.Data.SFTypeObj[tdata].TempImproveCost > -1 & self.game.Data.SFTypeObj[tdata].ModelRegime == self.game.Data.Turn)
          {
            num2 += 1;
            self.OptionsListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
            if (self.detailnr == tdata)
              num1 = num2;
          }
        }
      }
      if (num1 == -1)
        self.detailnr = -1;
      ListClass optionsListObj = self.OptionsListObj;
      let mut tlistselect: i32 = num1;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart1 =  new ListSubPartClass(optionsListObj, 8, 220, tlistselect, game, tHeader: "Subformationtype Models", tShowPair: true, tValueWidth: 100, tbackbitmap: ( local1), bbx: 110, bby: 5, overruleFont: ( local2));
      self.OptionsListId = self.AddSubPart( tsubpart1, 110, 5, 220, 176, 0);
      if (self.detailnr > -1)
      {
        DrawMod.DrawText( Expression, self.game.Data.SFTypeObj[self.detailnr].Name, self.game.GameFont3, 350, 10);
        let mut picSpriteId: i32 = self.game.Data.SFTypeObj[self.detailnr].PicSpriteID;
        if (self.game.Data.RegimeObj[self.game.Data.Turn].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[self.detailnr].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[self.detailnr].ExtraCode[index] == self.game.Data.RegimeObj[self.game.Data.Turn].ExtraGraphicUse)
              picSpriteId = self.game.Data.SFTypeObj[self.detailnr].ExtraPicSpriteID[index];
          }
        }
         let mut local3: &Graphics = &Expression;
        bitmap: Bitmap = BitmapStore.GetBitmap(picSpriteId);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawScaled( local3,  local4, 350, 30, 100, 75);
        DrawMod.DrawRectangle( Expression, 350, 30, 100, 75, 0, 0, 0,  byte.MaxValue);
        DrawMod.DrawRectangle( Expression, 351, 31, 98, 73,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        tsubpart1 =  new TextButtonPartClass("Info", 70, tBackbitmap: ( self.OwnBitmap), bbx: 350, bby: 115);
        self.B6Id = self.AddSubPart( tsubpart1, 350, 115, 70, 35, 1);
        tText: String = "";
        if (self.game.Data.SFTypeObj[self.detailnr].TempNewCost > -1)
        {
          str1: String = tText + "This is a concept. \r\n" + "Designing a model based on it is possible.\r\n";
          str2: String;
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
            str2 = str1 + "This costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempNewCost) + " PP.\r\n";
          else
            str2 = str1 + "This costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempNewCost) + " " + self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] + "\r\n";
          tText = str2 + "A new model will use " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempNewLevels) + " new core research levels and " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImprovementFields) + " improvement research fields.";
        }
        else if (self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost > -1)
        {
          str3: String = tText + "This is a model you can upgrade. " + "It uses " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelLevel)) + " core research levels. " + "Its Mark " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelMark)) + ". Its Version " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelVersion));
          let mut num5: i32 = 0;
          let mut researchCounter: i32 = self.game.Data.ResearchCounter;
          for (let mut index: i32 = 0; index <= researchCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[self.detailnr].ModelLastState[index] == 1)
            {
              num5 += 1;
              if (num5 == 1)
                str3 = str3 + "\r\n" + "This model has been improved with: ";
              if (num5 >= 2)
                str3 += ", ";
              str3 += self.game.Data.ResearchObj[index].Name;
            }
          }
          str4: String = str3 + "\r\n";
          str5: String;
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
            str5 = str4 + "Upgrade costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost) + " PP.\r\n";
          else
            str5 = str4 + "Upgrade costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost) + " " + self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] + "\r\n";
          tText = str5 + "A upgraded model will use " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempUpgradeLevels) + " new core research levels and " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImprovementFields) + " improvement research fields.";
          if (self.game.Data.SFTypeObj[self.detailnr].TempImproveCost > -1)
          {
            str6: String = tText + "\r\n" + "Improving it will use " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImprovementFields) + " new improvement research levels." + "\r\n";
            if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
              tText = str6 + "It can be improved upon. Improving this model costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImproveCost) + " PP.\r\n";
            else
              tText = str6 + "It can be improved upon. Improving this model costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImproveCost) + " " + self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] + "\r\n";
          }
        }
        else if (self.game.Data.SFTypeObj[self.detailnr].TempImproveCost > -1)
        {
          str7: String = tText + "This is a model you have developed..\r\n" + "It uses " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelLevel)) + " core research levels." + "\r\n" + "Its Mark " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelMark)) + ". Its Version " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelVersion));
          let mut num6: i32 = 0;
          let mut researchCounter: i32 = self.game.Data.ResearchCounter;
          for (let mut index: i32 = 0; index <= researchCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[self.detailnr].ModelLastState[index] == 1)
            {
              num6 += 1;
              if (num6 == 1)
                str7 = str7 + "\r\n" + "This model has been improved with: ";
              if (num6 >= 2)
                str7 += ", ";
              str7 += self.game.Data.ResearchObj[index].Name;
            }
          }
          str8: String = str7 + "\r\n" + "Improving it will use " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImprovementFields) + " new improvement research levels." + "\r\n";
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
            tText = str8 + "It can be improved upon. Improving this model costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImproveCost) + " PP.\r\n";
          else
            tText = str8 + "It can be improved upon. Improving this model costs " + Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].TempImproveCost) + " " + self.game.Data.RegimeSlotName[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] + "\r\n";
        }
        else if (self.game.Data.SFTypeObj[self.detailnr].ModelID > 0 & self.game.Data.SFTypeObj[self.detailnr].ModelRegime == self.game.Data.Turn)
        {
          str: String = tText + "This is a model you have developed..\r\n" + "It uses " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelLevel)) + " core research levels." + "\r\n" + "Its Mark " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelMark)) + ". Its Version " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.detailnr].ModelVersion));
          let mut num7: i32 = 0;
          let mut researchCounter: i32 = self.game.Data.ResearchCounter;
          for (let mut index: i32 = 0; index <= researchCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[self.detailnr].ModelLastState[index] == 1)
            {
              num7 += 1;
              if (num7 == 1)
                str = str + "\r\n" + "This model has been improved with: ";
              if (num7 >= 2)
                str += ", ";
              str += self.game.Data.ResearchObj[index].Name;
            }
          }
          tText = str + "\r\n";
        }
        tsubpart1 =  new TextAreaClass(self.game, 390, 7, Font::new("Times New Roman", 14f, FontStyle.Regular, GraphicsUnit.Pixel), "Technical Data", true, tText, Color.White, tbackbitmap: ( self.OwnBitmap), bbx: 460, bby: 30);
        self.B5Id = self.AddSubPart( tsubpart1, 460, 30, 390, 160, 0);
        let mut num8: i32 = -1;
        let mut num9: i32 = 0;
        if (self.game.Data.SFTypeObj[self.detailnr].TempNewCost > -1)
        {
          let mut num10: i32 = 0;
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
          {
            if (self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >= self.game.Data.SFTypeObj[self.detailnr].TempNewCost)
              num10 = 1;
          }
          else if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] >= self.game.Data.SFTypeObj[self.detailnr].TempNewCost)
            num10 = 1;
          if (num10 == 1)
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("New", 100, tBackbitmap: ( self.OwnBitmap), bbx: 850, bby: (30 + num8 * 40));
            self.B7Id = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 1);
          }
          else
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("New", 100, "You cannot pay the cost",  self.OwnBitmap, 850, 30 + num8 * 40, true);
            self.b7bid = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 0);
          }
        }
        if (self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost > -1)
        {
          let mut num11: i32 = 0;
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
          {
            if (self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >= self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost)
              num11 = 1;
          }
          else if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] >= self.game.Data.SFTypeObj[self.detailnr].TempUpgradeCost)
            num11 = 1;
          if (num11 == 1)
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("Upgrade", 100, tBackbitmap: ( self.OwnBitmap), bbx: 850, bby: (30 + num8 * 40));
            self.B8Id = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 1);
          }
          else
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("Upgrade", 100, "You cannot pay the cost",  self.OwnBitmap, 850, 30 + num8 * 40, true);
            self.b8bid = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 0);
          }
        }
        if (self.game.Data.SFTypeObj[self.detailnr].TempImproveCost > -1)
        {
          let mut num12: i32 = 0;
          if (self.game.Data.SFTypeObj[self.detailnr].ModelCostType == -1)
          {
            if (self.game.Data.RegimeObj[self.game.Data.Turn].ResPts >= self.game.Data.SFTypeObj[self.detailnr].TempImproveCost)
              num12 = 1;
          }
          else if (self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[self.game.Data.SFTypeObj[self.detailnr].ModelCostType] >= self.game.Data.SFTypeObj[self.detailnr].TempImproveCost)
            num12 = 1;
          if (num12 == 1)
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("Improve", 100, tBackbitmap: ( self.OwnBitmap), bbx: 850, bby: (30 + num8 * 40));
            self.B9Id = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 1);
          }
          else
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("Improve", 100, "You cannot pay the cost",  self.OwnBitmap, 850, 30 + num8 * 40, true);
            self.b9bid = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 0);
          }
        }
        if (self.game.Data.SFTypeObj[self.detailnr].TempFieldCount == 0)
        {
          num9 = 0;
          if ((uint) (-(self.game.Data.SFTypeObj[self.detailnr].TempAlterationCost > 0 & self.game.Data.SFTypeObj[self.detailnr].ModelVersion == 1 ? 1 : 0) & self.game.Data.SFTypeObj[self.detailnr].ModelMark) > 0U)
          {
            num8 += 1;
            tsubpart1 =  new TextButtonPartClass("Alter", 100, tBackbitmap: ( self.OwnBitmap), bbx: 850, bby: (30 + num8 * 40));
            self.B11Id = self.AddSubPart( tsubpart1, 850, 30 + num8 * 40, 100, 35, 1);
          }
          if (!self.game.Data.SFTypeObj[self.detailnr].ModelIsBase)
          {
            let mut num13: i32 = num8 + 1;
            tsubpart1 =  new TextButtonPartClass("Rename", 100, tBackbitmap: ( self.OwnBitmap), bbx: 850, bby: (30 + num13 * 40));
            self.B10Id = self.AddSubPart( tsubpart1, 850, 30 + num13 * 40, 100, 35, 1);
          }
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.B11Id)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 79, self.detailnr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B10Id)
            {
              str: String = Interaction.InputBox("Give new name for all members of this model family", "Shadow Empire : Planetary Conquest");
              if (Strings.Len(str) > 1)
              {
                let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
                for (let mut index2: i32 = 0; index2 <= sfTypeCounter; index2 += 1)
                {
                  if (self.game.Data.SFTypeObj[index2].ModelID == self.game.Data.SFTypeObj[self.detailnr].ModelID)
                  {
                    let mut modelItemType: i32 = self.game.Data.SFTypeObj[index2].ModelItemType;
                    if (modelItemType > -1)
                      self.game.Data.ItemTypeObj[modelItemType].Name = self.game.Data.ItemTypeObj[modelItemType].Name.Replace(self.game.Data.SFTypeObj[index2].ModelName, str);
                    self.game.Data.SFTypeObj[index2].Name = self.game.Data.SFTypeObj[index2].Name.Replace(self.game.Data.SFTypeObj[index2].ModelName, str);
                    self.game.Data.SFTypeObj[index2].ModelName = str;
                  }
                }
                self.game.HandyFunctionsObj.CalculateSFDesignCosts();
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num2: i32 =  Interaction.MsgBox( "Cancelled");
            }
            else
            {
              if (num1 == self.B9Id)
              {
                self.game.ProcessingObj.MakeSFTypeModelImprovement(self.detailnr);
                self.AutoChangeProdLine(self.game.Data.ItemTypeCounter);
                self.game.ProcessingObj.LocationProductionPrognosis();
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                self.optnr = 1;
                self.detailnr = self.game.Data.SFTypeCounter;
                self.game.HandyFunctionsObj.CalculateSFDesignCosts();
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B8Id)
              {
                self.game.ProcessingObj.MakeSFTypeModelUpgrade(self.detailnr);
                self.AutoChangeProdLine(self.game.Data.ItemTypeCounter);
                self.game.ProcessingObj.LocationProductionPrognosis();
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                self.optnr = 1;
                self.detailnr = self.game.Data.SFTypeCounter;
                self.game.HandyFunctionsObj.CalculateSFDesignCosts();
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B7Id)
              {
                self.game.ProcessingObj.MakeNewSFTypeModel(self.detailnr);
                self.AutoChangeProdLine(self.game.Data.ItemTypeCounter);
                self.game.ProcessingObj.LocationProductionPrognosis();
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 66);
                self.optnr = 1;
                self.detailnr = self.game.Data.SFTypeCounter;
                self.game.HandyFunctionsObj.CalculateSFDesignCosts();
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B6Id)
              {
                self.game.EditObj.TempProdList1 = self.optnr;
                self.game.EditObj.TempProdList2 = self.detailnr;
                self.game.EditObj.TempProdList3 = -1;
                self.game.EditObj.SFTypeSelected = self.detailnr;
                self.game.EditObj.SFSelected = -1;
                self.game.EditObj.CameFrom = 65;
                windowReturnClass.AddCommand(3, 8);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B5Id)
              {
                self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B1Id)
              {
                self.optnr = 1;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B2Id)
              {
                self.optnr = 2;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B3Id)
              {
                self.optnr = 3;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B4Id)
              {
                self.optnr = 4;
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.OptionsListId)
              {
                let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num3 > -1)
                {
                  self.detailnr = num3;
                  self.dostuff();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn AutoChangeProdLine(newi: i32)
    {
      SimpleList simpleList = SimpleList::new();
      let mut itemTypeCounter: i32 = self.game.Data.ItemTypeCounter;
      Number: i32;
      for (let mut itemtypenr: i32 = 0; itemtypenr <= itemTypeCounter; itemtypenr += 1)
      {
        if (itemtypenr == newi && self.game.Data.ItemTypeObj[itemtypenr].Blocks > -1)
        {
          let mut blocks: i32 = self.game.Data.ItemTypeObj[itemtypenr].Blocks;
          let mut locCounter: i32 = self.game.Data.LocCounter;
          for (let mut locnr: i32 = 0; locnr <= locCounter; locnr += 1)
          {
            if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn)
            {
              let mut index: i32 = 0;
              do
              {
                if (self.game.Data.LocObj[locnr].Production[index] == self.game.Data.ItemTypeObj[itemtypenr].Blocks && self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, itemtypenr).result)
                {
                  self.game.Data.LocObj[locnr].Production[index] = itemtypenr;
                  Number += 1;
                }
                index += 1;
              }
              while (index <= 3);
            }
          }
        }
      }
      if (Number <= 0)
        return;
      let mut num: i32 =  Interaction.MsgBox( ("Automatically switched " + Conversion.Str( Number) + " production line(s)."), Title: ( "Shadow Empire : Planetary Conquest"));
    }
  }
}
