// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFWindowClass2
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
  pub class SFWindowClass2 : WindowClass
  {
     TempText1: i32;
     temptext2: i32;
     temptext3: i32;
     temptext4: i32;
     temptext5: i32;
     temptext6: i32;
     temptext7: i32;
     temptext8: i32;
     temptext9: i32;
     temptext10: i32;
     TempText11: i32;
     temptext12: i32;
     temptext13: i32;
     temptext14: i32;
     temptext15: i32;
     temptext16: i32;
     temptext17: i32;
     temptext18: i32;
     temptext19: i32;
     temptext20: i32;
     TempText21: i32;
     temptext22: i32;
     temptext23: i32;
     temptext24: i32;
     temptext25: i32;
     temptext26: i32;
     temptext27: i32;
     temptext28: i32;
     temptext29: i32;
     temptext30: i32;
     TempText31: i32;
     temptext32: i32;
     temptext33: i32;
     temptext34: i32;
     temptext35: i32;
     temptext36: i32;
     temptext37: i32;
     temptext38: i32;
     temptext39: i32;
     temptext40: i32;
     temptext41: i32;
     temptext42: i32;
     temptext43: i32;
     temptext44: i32;
     temptext45: i32;
     temptext46: i32;
     LogoListId: i32;
     but1id: i32;
     but1textid: i32;
     but1bid: i32;
     hqbut0: i32;
     hqbut1: i32;
     hqbut2: i32;
     but2id: i32;
     but2textid: i32;
     but3id: i32;
     but3textid: i32;
     but4id: i32;
     but4textid: i32;
     but5id: i32;
     but5textid: i32;
     but6id: i32;
     but6textid: i32;
     but7id: i32;
     but7textid: i32;
     descid: i32;
     comparenr: i32;
     sliderid: i32;
     logolist2id: i32;
     logolist3id: i32;
     float tempBlink;
     unr: i32;
     sfnr: i32;
     sftyp: i32;
     detailnr: i32;
     detailnr2: i32;
     detailtype: i32;
     ammount: i32;
     bool hqreach;
     passenger: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ListClass OptionsList6Obj;
     combatListId: i32;
     ListClass combatListObj;
     combatList2Id: i32;
     ListClass combatList2Obj;
     StatTyp: i32;
     StatMode: i32;
     int[] ChainHq;
     HQselect: i32;
     infoid: i32;

    pub fn DoRefresh()
    {
      self.comparenr = self.game.EditObj.SFCompare;
      self.DoStuff();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            if (self.SubPartID[index] != self.LogoListId & self.SubPartID[index] != self.logolist3id)
              self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            break;
          }
        }
      }
    }

    pub SFWindowClass2( tGame: GameClass)
      : base( tGame, 880, 580, 8)
    {
      self.ChainHq = new int[3];
      self.tempBlink = 0.0f;
      self.sfnr = -1;
      self.comparenr = -1;
      self.game.EditObj.SFCompare = -1;
      if (self.game.EditObj.SFSelected > -1)
      {
        if (self.game.EditObj.SFSelected > self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFCount)
        {
          self.passenger = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].PassengerList[self.game.EditObj.SFSelected - (1 + self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFCount)];
          self.sfnr = -1;
          self.sftyp = -1;
        }
        else
        {
          self.sfnr = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SFList[self.game.EditObj.SFSelected];
          self.sftyp = self.game.Data.SFObj[self.sfnr].Type;
          self.unr = self.game.EditObj.UnitSelected;
          self.passenger = -1;
        }
      }
      else
      {
        self.sftyp = self.game.EditObj.SFTypeSelected;
        self.sfnr = -1;
      }
      self.DoStuff();
    }

    pub fn DoStuff()
    {
      self.ClearMouse();
      self.NewBackGroundAndClearAll(880, 580, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      let mut watermark: i32 = -1;
      index1: i32;
      if (self.sfnr > -1)
      {
        let mut type: i32 = self.game.Data.SFObj[self.sfnr].Type;
        let mut people: i32 = self.game.Data.SFObj[self.sfnr].People;
        watermark = self.game.Data.SFTypeObj[type].PicSpriteID;
        if (self.game.Data.RegimeObj[index1].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
          for (let mut index2: i32 = 0; index2 <= extraCounter; index2 += 1)
          {
            if (self.game.Data.SFTypeObj[type].ExtraCode[index2] == self.game.Data.RegimeObj[index1].ExtraGraphicUse)
              watermark = self.game.Data.SFTypeObj[type].ExtraPicSpriteID[index2];
          }
        }
        else if (people > -1 && self.game.Data.PeopleObj[people].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
          for (let mut index3: i32 = 0; index3 <= extraCounter; index3 += 1)
          {
            if (self.game.Data.SFTypeObj[type].ExtraCode[index3] == self.game.Data.PeopleObj[people].ExtraGraphicUse)
              watermark = self.game.Data.SFTypeObj[type].ExtraPicSpriteID[index3];
          }
        }
      }
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 880, 580, watermark);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (self.LogoListId > 0)
        self.RemoveSubPart(self.LogoListId);
      if (self.logolist2id > 0)
        self.RemoveSubPart(self.logolist2id);
      if (self.logolist3id > 0)
        self.RemoveSubPart(self.logolist3id);
      if (self.but1id > 0)
        self.RemoveSubPart(self.but1id);
      if (self.but1bid > 0)
        self.RemoveSubPart(self.but1bid);
      if (self.but1textid > 0)
        self.RemoveSubPart(self.but1textid);
      if (self.but7id > 0)
        self.RemoveSubPart(self.but7id);
      if (self.but7textid > 0)
        self.RemoveSubPart(self.but7textid);
      if (self.descid > 0)
        self.RemoveSubPart(self.descid);
      let mut typ: i32 = self.sftyp;
      let mut pplnr: i32 = self.game.Data.Turn <= -1 ? 0 : self.game.Data.RegimeObj[self.game.Data.Turn].People;
      str1: String = "";
      if (self.sfnr > -1)
      {
        typ = self.game.Data.SFObj[self.sfnr].Type;
        pplnr = self.game.Data.SFObj[self.sfnr].People;
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime == self.game.Data.Turn)
        {
          str1 = Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Qty)) + "x ";
          if (self.game.Data.SFTypeObj[typ].Ratio > 1)
            str1 = str1 + " " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[typ].Ratio)) + "x ";
        }
      }
      regnr: i32;
      num1: i32;
      num2: i32;
      Rectangle rectangle1;
      Rectangle trect;
      if (typ > -1)
      {
        name: String = self.game.Data.SFTypeObj[typ].Name;
        if (self.game.Data.RegimeObj[index1].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index4: i32 = 0; index4 <= extraCounter; index4 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index4] == self.game.Data.RegimeObj[index1].ExtraGraphicUse)
            {
              index5: i32;
              name = self.game.Data.SFTypeObj[index5].ExtraName[index4];
            }
          }
        }
        else if (pplnr > -1 & self.sfnr > -1 && self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index6: i32 = 0; index6 <= extraCounter; index6 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index6] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              name = self.game.Data.SFTypeObj[typ].ExtraName[index6];
          }
        }
        str2: String = str1 + name;
        SizeF sizeF1 = SizeF::new();
        if (self.comparenr == -1)
        {
          SizeF sizeF2 = graphics.MeasureString(name + " trooptype details", self.game.MarcFont1);
          DrawMod.DrawTextColouredMarc( graphics, name + " trooptype details", self.game.MarcFont1, 417 -  Math.Round( (sizeF2.Width / 2f)), 19, Color.White);
        }
        index1 = 0;
        regnr = self.sfnr <= -1 ? self.game.Data.Turn : self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime;
        let mut picSpriteId: i32 = self.game.Data.SFTypeObj[typ].PicSpriteID;
        if (self.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index7: i32 = 0; index7 <= extraCounter; index7 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index7] == self.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              picSpriteId = self.game.Data.SFTypeObj[typ].ExtraPicSpriteID[index7];
          }
        }
        else if (pplnr > -1 && self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index8: i32 = 0; index8 <= extraCounter; index8 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index8] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              picSpriteId = self.game.Data.SFTypeObj[typ].ExtraPicSpriteID[index8];
          }
        }
        if (typ > -1)
        {
          num1 = 450;
          let mut num3: i32 = 20;
          if (self.comparenr > -1)
          {
            num1 = 160;
            num2 = 450;
          }
          if (self.comparenr > -1)
          {
            let mut sidewaysSpriteId1: i32 = self.game.Data.SFTypeObj[typ].SidewaysSpriteID;
            DrawMod.DrawBlockGradient2( graphics, num1 + 60, num3, 139, 79, self.game.MarcCol1, self.game.MarcCol2);
             let mut local1: &Graphics = &graphics;
            bitmap1: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId1);
             let mut local2: &Bitmap = &bitmap1;
            Rectangle rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(sidewaysSpriteId1), BitmapStore.Getheight(sidewaysSpriteId1));
            let mut srcrect1: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num1 + 60, num3, 140, 80);
            let mut destrect1: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  graphics, num1 + 60, num3, 140, 80, -1, -1);
            let mut sidewaysSpriteId2: i32 = self.game.Data.SFTypeObj[self.comparenr].SidewaysSpriteID;
            DrawMod.DrawBlockGradient2( graphics, num2 + 60, num3, 139, 79, self.game.MarcCol1, self.game.MarcCol2);
             let mut local3: &Graphics = &graphics;
            bitmap2: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId2);
             let mut local4: &Bitmap = &bitmap2;
            rectangle1 = Rectangle::new(0, 0, BitmapStore.GetWidth(sidewaysSpriteId2), BitmapStore.Getheight(sidewaysSpriteId2));
            let mut srcrect2: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(num2 + 60, num3, 140, 80);
            let mut destrect2: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  graphics, num2 + 60, num3, 140, 80, -1, -1);
          }
          else if (self.comparenr == -1)
          {
            let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[typ].SidewaysSpriteID;
            if (self.game.Data.SFTypeObj[typ].Theater == 0 || self.game.Data.SFTypeObj[typ].Theater == 1 || self.game.Data.SFTypeObj[typ].Theater != 2)
              ;
            let mut x: i32 = num1 - 310;
            let mut y: i32 = 77;
            let mut index9: i32 =  Math.Round( self.game.Data.RuleVar[873]);
            let mut index10: i32 = 0;
            if ( self.game.Data.RuleVar[848] > 0.0 & self.game.Data.SFTypeObj[typ].Theater == 2)
            {
              index9 =  Math.Round( self.game.Data.RuleVar[848]);
              index10 = 0;
            }
            if ( self.game.Data.RuleVar[872] > 0.0 & self.game.Data.SFTypeObj[typ].Theater == 1)
            {
              index9 =  Math.Round( self.game.Data.RuleVar[872]);
              index10 = 0;
            }
            let mut nr1: i32 = self.game.Data.LandscapeTypeObj[index9].BasicPicID[index10];
             let mut local5: &Graphics = &graphics;
            bitmap3: Bitmap = BitmapStore.GetBitmap(nr1);
             let mut local6: &Bitmap = &bitmap3;
            rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr1));
            let mut srcrect3: &Rectangle = &rectangle1
            trect = Rectangle::new(x, y, 280, 160);
            let mut destrect3: &Rectangle = &trect
            DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
            let mut nr2: i32 = self.game.Data.LandscapeTypeObj[index9].SidewaysSPriteID1[index10];
             let mut local7: &Graphics = &graphics;
            bitmap4: Bitmap = BitmapStore.GetBitmap(nr2);
             let mut local8: &Bitmap = &bitmap4;
            rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr2));
            let mut srcrect4: &Rectangle = &rectangle1
            trect = Rectangle::new(x, y, 280, 160);
            let mut destrect4: &Rectangle = &trect
            DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
            if (self.game.Data.SFTypeObj[typ].Theater != 2)
            {
              let mut nr3: i32 = self.game.Data.LandscapeTypeObj[index9].SidewaysSPriteID2[index10];
               let mut local9: &Graphics = &graphics;
              bitmap5: Bitmap = BitmapStore.GetBitmap(nr3);
               let mut local10: &Bitmap = &bitmap5;
              rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr3));
              let mut srcrect5: &Rectangle = &rectangle1
              trect = Rectangle::new(x, y, 280, 160);
              let mut destrect5: &Rectangle = &trect
              DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
            }
             let mut local11: &Graphics = &graphics;
            bitmap6: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
             let mut local12: &Bitmap = &bitmap6;
            rectangle1 = Rectangle::new(0, 0, BitmapStore.GetWidth(sidewaysSpriteId), BitmapStore.Getheight(sidewaysSpriteId));
            let mut srcrect6: &Rectangle = &rectangle1
            trect = Rectangle::new(x, y, 280, 160);
            let mut destrect6: &Rectangle = &trect
            DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
            if (self.game.Data.SFTypeObj[typ].Theater != 2)
            {
              let mut nr4: i32 = self.game.Data.LandscapeTypeObj[index9].SidewaysSPriteID3[index10];
               let mut local13: &Graphics = &graphics;
              bitmap7: Bitmap = BitmapStore.GetBitmap(nr4);
               let mut local14: &Bitmap = &bitmap7;
              rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr4));
              let mut srcrect7: &Rectangle = &rectangle1
              trect = Rectangle::new(x, y, 280, 160);
              let mut destrect7: &Rectangle = &trect
              DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
            }
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  graphics, x, y, 280, 160, -1, -1);
          }
        }
      }
      str3: String = self.game.Data.SFTypeObj[typ].Description;
      if (Strings.InStr(str3, "[tab]") == 0)
        str3 = "[tab]" + self.game.Data.SFTypeObj[typ].Name + " : Troop type description," + str3 + "[/tab]";
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 560,  Math.Round(Conversion.Int(160.0 / 17.0)), self.game.MarcFont8, str3, 17,  self.OwnBitmap, 300, 290);
      self.descid = self.AddSubPart( tsubpart1, 300, 290, 560,  Math.Round((Conversion.Int(160.0 / 17.0) + 1.0) * 17.0), 0);
      StringListClass tListobj1 = new StringListClass(1);
      let mut index11: i32 = -1;
      let mut num4: i32 = -1;
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      tListobj1.TempColumBmp[0] = Conversions.ToString(self.ReturnSFSpriteNr(typ, regnr, pplnr));
      tListobj1.ColumnName[1] = self.game.Data.SFTypeObj[typ].Ratio <= 1 ? Strings.UCase(self.game.Data.SFTypeObj[typ].Name) : Strings.UCase(self.game.Data.SFTypeObj[typ].Name) + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[typ].Ratio)) + "x)";
      let mut index12: i32 = 0;
      do
      {
        if (Strings.Len(self.game.Data.SFTypeObj[typ].LogoString[index12]) > 0 && Operators.CompareString(self.game.Data.SFTypeObj[typ].LogoString[index12], "0", false) != 0)
        {
          if ((num4 + 10) % 2 > 0)
          {
            index11 += 1;
            num4 += 1;
            tListobj1.AddRow(tListobj1.Length);
            if ( self.game.Data.RuleVar[947] == 1.0)
            {
              if (Strings.Len(self.game.Data.TempString[1000 + index12]) > 0)
              {
                tListobj1.TempBmp[index11, 0] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index12])];
                tListobj1.TempDesc[index11, 0] = self.game.Data.TempString[1100 + index12];
              }
            }
            else if (Strings.Len(self.game.Data.TempString[1000 + index12]) > 0)
            {
              tListobj1.TempBmp[index11, 0] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index12])];
              tListobj1.TempDesc[index11, 0] = self.game.Data.TempString[1100 + index12];
            }
            tListobj1.Data[index11, 1] = self.game.Data.SFTypeObj[typ].LogoString[index12];
            tListobj1.TempDesc[index11, 1] = self.game.Data.TempString[1100 + index12];
          }
          else
          {
            num4 += 1;
            if ( self.game.Data.RuleVar[947] == 1.0)
            {
              if (Strings.Len(self.game.Data.TempString[1000 + index12]) > 0)
              {
                tListobj1.TempBmp[index11, 2] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index12])];
                tListobj1.TempDesc[index11, 2] = self.game.Data.TempString[1100 + index12];
              }
            }
            else if (Strings.Len(self.game.Data.TempString[1000 + index12]) > 0)
            {
              tListobj1.TempBmp[index11, 2] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index12])];
              tListobj1.TempDesc[index11, 2] = self.game.Data.TempString[1100 + index12];
            }
            tListobj1.Data[index11, 3] = self.game.Data.SFTypeObj[typ].LogoString[index12];
            tListobj1.TempDesc[index11, 3] = self.game.Data.TempString[1100 + index12];
          }
        }
        index12 += 1;
      }
      while (index12 <= 99);
      let mut tlistsize1: i32 = 4;
      let mut num5: i32 = 110;
      if (self.comparenr == -1)
        num5 = 77;
      let mut tsubpart2: SubPartClass =  new MatrixSubPartClass(tListobj1, tlistsize1, 260, -1, -1, self.game, tbackbitmap: ( self.OwnBitmap), bbx: num1, bby: num5, trowheight: 26, tfontsize: 18, tfontoffsety: 2, tnolines: true, tMarcy: true, tMinColValue: 40);
      self.LogoListId = self.AddSubPart( tsubpart2, num1, num5, 260, (tlistsize1 + 3) * 26, 0);
      if (self.comparenr > -1)
      {
        StringListClass tListobj2 = new StringListClass(1);
        let mut index13: i32 = -1;
        let mut num6: i32 = -1;
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        tListobj2.TempColumBmp[0] = Conversions.ToString(self.ReturnSFSpriteNr(self.comparenr, regnr, pplnr));
        tListobj2.ColumnName[1] = self.game.Data.SFTypeObj[self.comparenr].Ratio <= 1 ? Strings.UCase(self.game.Data.SFTypeObj[self.comparenr].Name) : Strings.UCase(self.game.Data.SFTypeObj[self.comparenr].Name) + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Ratio)) + "x)";
        let mut index14: i32 = 0;
        do
        {
          if (Strings.Len(self.game.Data.SFTypeObj[self.comparenr].LogoString[index14]) > 0 && Operators.CompareString(self.game.Data.SFTypeObj[self.comparenr].LogoString[index14], "0", false) != 0)
          {
            if ((num6 + 10) % 2 > 0)
            {
              index13 += 1;
              num6 += 1;
              tListobj2.AddRow(tListobj2.Length);
              if ( self.game.Data.RuleVar[947] == 1.0)
              {
                if (Strings.Len(self.game.Data.TempString[1000 + index14]) > 0)
                {
                  tListobj2.TempBmp[index13, 0] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index14])];
                  tListobj2.TempDesc[index13, 0] = self.game.Data.TempString[1100 + index14];
                }
              }
              else if (Strings.Len(self.game.Data.TempString[1000 + index14]) > 0)
              {
                tListobj2.TempBmp[index13, 0] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index14])];
                tListobj2.TempDesc[index13, 0] = self.game.Data.TempString[1100 + index14];
              }
              tListobj2.Data[index13, 1] = self.game.Data.SFTypeObj[self.comparenr].LogoString[index14];
              tListobj2.TempDesc[index13, 1] = self.game.Data.TempString[1100 + index14];
            }
            else
            {
              num6 += 1;
              if ( self.game.Data.RuleVar[947] == 1.0)
              {
                if (Strings.Len(self.game.Data.TempString[1000 + index14]) > 0)
                {
                  tListobj2.TempBmp[index13, 2] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index14])];
                  tListobj2.TempDesc[index13, 2] = self.game.Data.TempString[1100 + index14];
                }
              }
              else if (Strings.Len(self.game.Data.TempString[1000 + index14]) > 0)
              {
                tListobj2.TempBmp[index13, 2] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index14])];
                tListobj2.TempDesc[index13, 2] = self.game.Data.TempString[1100 + index14];
              }
              tListobj2.Data[index13, 3] = self.game.Data.SFTypeObj[self.comparenr].LogoString[index14];
              tListobj2.TempDesc[index13, 3] = self.game.Data.TempString[1100 + index14];
            }
          }
          index14 += 1;
        }
        while (index14 <= 99);
        let mut tlistsize2: i32 = 4;
        let mut tsubpart3: SubPartClass =  new MatrixSubPartClass(tListobj2, tlistsize2, 260, -1, -1, self.game, tbackbitmap: ( self.OwnBitmap), bbx: num2, bby: 110, trowheight: 26, tfontsize: 18, tfontoffsety: 2, tnolines: true, tMarcy: true, tMinColValue: 40);
        self.logolist3id = self.AddSubPart( tsubpart3, num2, 110, 260, (tlistsize2 + 3) * 26, 0);
      }
      if (self.sfnr > -1)
      {
        Coordinate reconMinusHide;
        if (self.unr > -1)
        {
          if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn | self.game.Data.Round == 0)
            reconMinusHide.x = 3;
          else
            reconMinusHide = self.game.HandyFunctionsObj.GetReconMinusHide(self.unr, self.game.Data.Turn);
        }
        else
          reconMinusHide.x = 3;
        if (reconMinusHide.x >= 2)
        {
          self.OptionsList3Obj = ListClass::new();
          self.OptionsList3Obj.add("People", -1, self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].Name);
          float num7 = self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].BattleForMod[self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].People].PeopleGroup];
          if ( num7 > 1.0 |  num7 < 1.0)
          {
            let mut Number: i32 =  Math.Round( ((num7 - 1f) * 100f));
            tvalue: String = Strings.Trim(Conversion.Str( Number)) + "%";
            if (Number >= 0)
              tvalue = "+" + tvalue;
            self.OptionsList3Obj.add("People Combat Bonus", -1, tvalue);
          }
          self.OptionsList3Obj.add("Class", -1, self.game.Data.TempString[400 + self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].UnitGroup]);
          if (reconMinusHide.x == 3)
          {
            self.OptionsList3Obj.add("Qty", -1, Strings.Trim(Conversion.Str( (self.game.Data.SFObj[self.sfnr].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Ratio))));
            self.OptionsList3Obj.add("Readiness", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Rdn)));
            self.OptionsList3Obj.add("Morale", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Mor)));
            self.OptionsList3Obj.add("Experience", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Xp)));
            self.OptionsList3Obj.add("Entrenchment", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].CurrentEntrench)));
            self.OptionsList3Obj.add("Action Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Ap)));
            self.OptionsList3Obj.add("Engineer Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].EP)));
            self.OptionsList3Obj.add("Ratio", -1, "x" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Ratio)));
            self.OptionsList3Obj.add("Individuals", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Qty)));
            self.OptionsList3Obj.add("Weight/Carry", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Weight)) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].CarryCap)));
          }
          if (self.OptionsList3Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
          }
          else
          {
            ListClass optionsList3Obj = self.OptionsList3Obj;
            let mut game: GameClass = self.game;
             local15: Bitmap =  self.OwnBitmap;
            font: Font =  null;
             local16: Font =  font;
            let mut tsubpart4: SubPartClass =  new ListSubPartClass(optionsList3Obj, 11, 260, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local15), bbx: 20, bby: 308, tMarcStyle: true, overruleFont: ( local16));
            self.OptionsList3Id = self.AddSubPart( tsubpart4, 20, 308, 260, 192, 0);
          }
          DrawMod.DrawTextColouredMarc( graphics, "SELECTED TROOPS STATS", self.game.MarcFont5, 90, 291, Color.White);
          rectangle1 = Rectangle::new(20, 335, 290, 144);
          trect = rectangle1;
          self.AddMouse( trect, "", "The troops in the slot you clicked\r\nhave their own detailed stats.");
        }
      }
      let mut tsubpart5: SubPartClass =  new TextButtonPartClass("OK", 150, "Click to return to main screen.",  self.OwnBitmap,  Math.Round( self.OwnBitmap.Width / 2.0 + 15.0), 515, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but1id = self.AddSubPart( tsubpart5,  Math.Round( self.OwnBitmap.Width / 2.0 + 15.0), 515, 150, 40, 1);
      let mut tsubpart6: SubPartClass =  new TextButtonPartClass("COMPARE", 150, "Click to select another trooptype to compare stats with.",  self.OwnBitmap,  Math.Round( self.OwnBitmap.Width / 2.0 - 175.0), 515, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but7id = self.AddSubPart( tsubpart6,  Math.Round( self.OwnBitmap.Width / 2.0 - 175.0), 515, 150, 40, 1);
    }

    pub object ReturnSFSpriteNr(typ: i32, regnr: i32, pplnr: i32)
    {
      let mut symbolSpriteId: i32 = self.game.Data.SFTypeObj[typ].SymbolSpriteID;
      if (regnr > -1)
      {
        if (self.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
        else if (pplnr > -1 && self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index: i32 = 0; index <= extraCounter; index += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
          }
        }
      }
      else if (self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
      {
        let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
        for (let mut index: i32 = 0; index <= extraCounter; index += 1)
        {
          if (self.game.Data.SFTypeObj[typ].ExtraCode[index] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
            symbolSpriteId = self.game.Data.SFTypeObj[typ].ExtraSymbolSpriteID[index];
        }
      }
      return  symbolSpriteId;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 40)
        {
          self.SubPartList[self.SubpartNr(self.descid)].ShiftDown();
          self.SubPartFlag[self.SubpartNr(self.descid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 38)
        {
          self.SubPartList[self.SubpartNr(self.descid)].ShiftUp();
          self.SubPartFlag[self.SubpartNr(self.descid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 37)
        {
          self.SubPartList[self.SubpartNr(self.descid)].ShiftLeft();
          self.SubPartFlag[self.SubpartNr(self.descid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 39)
        {
          self.SubPartList[self.SubpartNr(self.descid)].ShiftRight();
          self.SubPartFlag[self.SubpartNr(self.descid)] = true;
          windowReturnClass.SetFlag(true);
        }
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
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            num2: i32;
            if (num1 == self.LogoListId)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList3Id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.logolist2id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.logolist3id)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.descid)
            {
              num2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.but1id || num1 == self.but1bid)
            {
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 != self.but7id)
              return windowReturnClass;
            Form3::new( self.formref).Initialize(self.game.Data, 74, -1);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
