// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFWindowClass3
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SFWindowClass3 : WindowClass
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

    pub SFWindowClass3( tGame: GameClass)
      : base( tGame, 880, 768, 8)
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
      marcCol1: Color = self.game.MarcCol1;
      marcCol2: Color = self.game.MarcCol2;
      c2_1: Color = Color.FromArgb(200, 40, 25, 15);
      c1: Color = Color.FromArgb(100, 80, 50, 29);
      c2_2: Color = Color.FromArgb(200, 120, 75, 45);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(880, 768, -1);
      Graphics graphics1 = Graphics.FromImage((Image) self.OwnBitmap);
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
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics1, 0, 0, 880, 768, watermark);
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
      str2: String;
      regnr: i32;
      num1: i32;
      num2: i32;
      num3: i32;
      Rectangle rectangle1;
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
        str2 = str1 + name;
        SizeF sizeF = SizeF::new();
        index1 = 0;
        regnr = self.sfnr <= -1 ? self.game.Data.Turn : self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime;
        num1 = self.game.Data.SFTypeObj[typ].PicSpriteID;
        if (self.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index7: i32 = 0; index7 <= extraCounter; index7 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index7] == self.game.Data.RegimeObj[regnr].ExtraGraphicUse)
              num1 = self.game.Data.SFTypeObj[typ].ExtraPicSpriteID[index7];
          }
        }
        else if (pplnr > -1 && self.game.Data.PeopleObj[pplnr].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 = self.game.Data.SFTypeObj[typ].ExtraCounter;
          for (let mut index8: i32 = 0; index8 <= extraCounter; index8 += 1)
          {
            if (self.game.Data.SFTypeObj[typ].ExtraCode[index8] == self.game.Data.PeopleObj[pplnr].ExtraGraphicUse)
              num1 = self.game.Data.SFTypeObj[typ].ExtraPicSpriteID[index8];
          }
        }
        if (typ > -1)
        {
          num2 = 540;
          let mut num4: i32 = 20;
          if (self.comparenr > -1)
          {
            num2 = 160;
            num3 = 450;
          }
          if (self.comparenr > -1)
          {
            let mut sidewaysSpriteId1: i32 = self.game.Data.SFTypeObj[typ].SidewaysSpriteID;
            DrawMod.DrawBlockGradient2( graphics1, num2 + 60, num4, 139, 79, c1, c2_2);
             let mut local1: &Graphics = &graphics1;
            bitmap1: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId1);
             let mut local2: &Bitmap = &bitmap1;
            let mut sftypenr: i32 = typ;
            let mut ppl1: i32 = pplnr;
            let mut tx1: i32 = num2 + 60;
            let mut ty1: i32 = num4;
            DrawMod.DrawWithArtCode( local1,  local2, 420, 240, sftypenr, ppl1, tx1, ty1, 140, 80);
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  graphics1, num2 + 60, num4, 140, 80, -1, -1);
            let mut sidewaysSpriteId2: i32 = self.game.Data.SFTypeObj[self.comparenr].SidewaysSpriteID;
            DrawMod.DrawBlockGradient2( graphics1, num3 + 60, num4, 139, 79, c1, c2_2);
             let mut local3: &Graphics = &graphics1;
            bitmap2: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId2);
             let mut local4: &Bitmap = &bitmap2;
            let mut comparenr: i32 = self.comparenr;
            let mut ppl2: i32 = pplnr;
            let mut tx2: i32 = num3 + 60;
            let mut ty2: i32 = num4;
            DrawMod.DrawWithArtCode( local3,  local4, 420, 240, comparenr, ppl2, tx2, ty2, 140, 80);
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  graphics1, num3 + 60, num4, 140, 80, -1, -1);
          }
          else if (self.comparenr == -1)
          {
            let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[typ].SidewaysSpriteID;
            if (self.game.Data.SFTypeObj[typ].Theater == 0 || self.game.Data.SFTypeObj[typ].Theater == 1 || self.game.Data.SFTypeObj[typ].Theater != 2)
              ;
            let mut num5: i32 = 74;
            let mut num6: i32 = 36;
            Rectangle rectangle2;
            if (self.game.SelectX > -1 & self.sfnr > -1 & self.game.SelectY > -1)
            {
              let mut landscapeType: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType;
              let mut newGfxSkyEventPic: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxSkyEventPic;
              let mut newGfxSkyX: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxSkyX;
              let mut newGfxSkyY: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxSkyY;
              if (newGfxSkyEventPic > -1)
              {
                 let mut local5: &Graphics = &graphics1;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[newGfxSkyEventPic]);
                 let mut local6: &Bitmap = &bitmap;
                rectangle2 = Rectangle::new(newGfxSkyX * 420, newGfxSkyY * 240, 420, 240);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(num5, num6, 420, 240);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
              }
              let mut backgroundEventPic1: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxBackgroundEventPic;
              let mut newGfxBackgroundX: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxBackgroundX;
              let mut newGfxBackgroundY: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxBackgroundY;
              if (backgroundEventPic1 > -1)
              {
                 let mut local7: &Graphics = &graphics1;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[backgroundEventPic1]);
                 let mut local8: &Bitmap = &bitmap;
                rectangle1 = Rectangle::new(newGfxBackgroundX * 420, newGfxBackgroundY * 240, 420, 240);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(num5, num6, 420, 240);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
              }
              let mut backgroundEventPic2: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherBackgroundEventPic;
              let mut weatherBackgroundX: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherBackgroundX;
              let mut weatherBackgroundY: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherBackgroundY;
              if (backgroundEventPic2 > -1)
              {
                 let mut local9: &Graphics = &graphics1;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[backgroundEventPic2]);
                 let mut local10: &Bitmap = &bitmap;
                rectangle1 = Rectangle::new(weatherBackgroundX * 420, weatherBackgroundY * 240, 420, 240);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(num5, num6, 420, 240);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
              }
            }
            else
            {
              DrawMod.DrawBlockGradient2( graphics1, num5, num6, 419, 239, Color.Transparent, c2_2);
              DrawMod.DrawBlockGradient2( graphics1, num5, num6 + 160, 419, 79, c1, c2_1);
            }
            Graphics graphics2 = graphics1;
            rectangle1 = Rectangle::new(num5, num6, 420, 240);
            let mut rect1: &Rectangle = &rectangle1
            graphics2.SetClip(rect1);
            if (self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].Mirror)
            {
              Matrix matrix = Matrix::new();
              matrix.Scale(-1f, 1f);
              matrix.Translate( -(2 * num5 + 420), 0.0f);
              graphics1.Transform = matrix;
               let mut local11: &Graphics = &graphics1;
              bitmap: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
               let mut local12: &Bitmap = &bitmap;
              let mut sftypenr: i32 = typ;
              let mut ppl: i32 = pplnr;
              let mut tx: i32 = num5;
              let mut ty: i32 = num6 + 28;
              DrawMod.DrawWithArtCode( local11,  local12, 420, 240, sftypenr, ppl, tx, ty, 420, 240);
              graphics1.ResetTransform();
            }
            else
            {
               let mut local13: &Graphics = &graphics1;
              bitmap: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
               let mut local14: &Bitmap = &bitmap;
              let mut sftypenr: i32 = typ;
              let mut ppl: i32 = pplnr;
              let mut tx: i32 = num5;
              let mut ty: i32 = num6 + 28;
              DrawMod.DrawWithArtCode( local13,  local14, 420, 240, sftypenr, ppl, tx, ty, 420, 240);
            }
            Graphics graphics3 = graphics1;
            rectangle1 = Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height);
            let mut rect2: &Rectangle = &rectangle1
            graphics3.SetClip(rect2);
            if (self.game.SelectX > -1 & self.sfnr > -1 & self.game.SelectY > -1)
            {
              let mut landscapeType: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType;
              let mut foregroundEventPic1: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxForegroundEventPic;
              let mut newGfxForegroundX: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxForegroundX;
              let mut newGfxForegroundY: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxForegroundY;
              if (foregroundEventPic1 > -1)
              {
                 let mut local15: &Graphics = &graphics1;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[foregroundEventPic1]);
                 let mut local16: &Bitmap = &bitmap;
                rectangle1 = Rectangle::new(newGfxForegroundX * 420, newGfxForegroundY * 240, 420, 240);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(num5, num6, 420, 240);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
              }
              let mut foregroundEventPic2: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherForegroundEventPic;
              let mut weatherForegroundX: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherForegroundX;
              let mut weatherForegroundY: i32 = self.game.Data.LandscapeTypeObj[landscapeType].NewGfxWeatherForegroundY;
              if (foregroundEventPic2 > -1)
              {
                 let mut local17: &Graphics = &graphics1;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[foregroundEventPic2]);
                 let mut local18: &Bitmap = &bitmap;
                rectangle1 = Rectangle::new(weatherForegroundX * 420, weatherForegroundY * 240, 420, 240);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(num5, num6, 420, 240);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2( local17,  local18, srcrect, destrect);
              }
            }
            if (self.comparenr == -1)
              sizeF = graphics1.MeasureString(name, self.game.MarcFont1);
          }
        }
      }
      str3: String = self.game.Data.SFTypeObj[typ].Description;
      if (Strings.InStr(str3, "[tab]") == 0)
        str3 = "[tab]" + self.game.Data.SFTypeObj[typ].Name + " (" + self.game.Data.TempString[400 + self.game.Data.SFTypeObj[typ].UnitGroup] + ")," + str3 + "[/tab]";
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 560,  Math.Round(Conversion.Int(160.0 / 17.0)), self.game.MarcFont8, str3, 17,  self.OwnBitmap, 300, 290);
      self.descid = self.AddSubPart( tsubpart1, 300, 290, 560,  Math.Round((Conversion.Int(160.0 / 17.0) + 2.0) * 17.0), 0);
      tvalue2_1: String = "";
      self.OptionsList2Obj = ListClass::new();
      tvalue1: String = self.game.Data.SFTypeObj[typ].SupplyCarry.ToString();
      if (self.comparenr > -1)
        tvalue2_1 = self.game.Data.SFTypeObj[self.comparenr].SupplyCarry.ToString();
      self.OptionsList2Obj.add("Supply max organic storage", -1, tvalue1, tvalue2_1);
      tvalue2: String = self.game.Data.SFTypeObj[typ].BasicSupplyNeed.ToString();
      if (self.comparenr > -1)
        tvalue2_1 = self.game.Data.SFTypeObj[self.comparenr].BasicSupplyNeed.ToString();
      self.OptionsList2Obj.add("Basic Supply use", -1, tvalue2, tvalue2_1);
      let mut num7: i32 = 10;
      if (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].EndCombatRound > 0 & self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].EndCombatRound < num7)
        num7 = self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].EndCombatRound - self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].StartCombatRound;
      tvalue3: String = Math.Round( self.game.Data.SFTypeObj[typ].SupplyForAttack /  num7, 2).ToString();
      let mut num8: i32 = 10;
      double num9;
      if (self.comparenr > -1)
      {
        if (self.game.Data.SFTypeObj[self.comparenr].EndCombatRound > 0 & self.game.Data.SFTypeObj[self.comparenr].EndCombatRound < num7)
          num8 = self.game.Data.SFTypeObj[self.comparenr].EndCombatRound - self.game.Data.SFTypeObj[self.comparenr].StartCombatRound;
        num9 = Math.Round( self.game.Data.SFTypeObj[self.comparenr].SupplyForAttack /  num8, 2);
        tvalue2_1 = num9.ToString();
      }
      self.OptionsList2Obj.add("Supply per Off Combat round", -1, tvalue3, tvalue2_1);
      num9 = Math.Round( self.game.Data.SFTypeObj[typ].SupplyForAttackDef /  num7, 2);
      tvalue4: String = num9.ToString();
      if (self.comparenr > -1)
      {
        num9 = Math.Round( self.game.Data.SFTypeObj[self.comparenr].SupplyForAttackDef /  num8, 2);
        tvalue2_1 = num9.ToString();
      }
      self.OptionsList2Obj.add("Supply per Def Combat round", -1, tvalue4, tvalue2_1);
      tvalue5: String = self.game.Data.SFTypeObj[typ].SupplyMaxIn.ToString();
      if (self.comparenr > -1)
        tvalue2_1 = self.game.Data.SFTypeObj[self.comparenr].SupplyMaxIn.ToString();
      self.OptionsList2Obj.add("Supply max Replenish per turn", -1, tvalue5, tvalue2_1);
      tvalue6: String = "";
      tvalue2_2: String = "";
      self.OptionsList2Obj.add("----", -1, tvalue6, tvalue2_2);
      tvalue7: String = self.game.Data.SFTypeObj[typ].FuelCarry.ToString();
      if (self.comparenr > -1)
        tvalue2_2 = self.game.Data.SFTypeObj[self.comparenr].FuelCarry.ToString();
      self.OptionsList2Obj.add("Fuel max organic storage", -1, tvalue7, tvalue2_2);
      tvalue8: String = self.game.Data.SFTypeObj[typ].FuelForMove.ToString();
      if (self.comparenr > -1)
        tvalue2_2 = self.game.Data.SFTypeObj[self.comparenr].FuelForMove.ToString();
      self.OptionsList2Obj.add("Fuel for 10AP Move", -1, tvalue8, tvalue2_2);
      tvalue9: String = self.game.Data.SFTypeObj[typ].FuelForAttack.ToString();
      if (self.comparenr > -1)
        tvalue2_2 = self.game.Data.SFTypeObj[self.comparenr].FuelForAttack.ToString();
      self.OptionsList2Obj.add("Fuel per Off Combat Round", -1, tvalue9, tvalue2_2);
      tvalue10: String = self.game.Data.SFTypeObj[typ].FuelForAttackDef.ToString();
      if (self.comparenr > -1)
        tvalue2_2 = self.game.Data.SFTypeObj[self.comparenr].FuelForAttackDef.ToString();
      self.OptionsList2Obj.add("Fuel per Def Combat Round", -1, tvalue10, tvalue2_2);
      if (self.OptionsList2Id > 0)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, -1);
        self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
      }
      else
      {
        ListClass optionsList2Obj = self.OptionsList2Obj;
        let mut game: GameClass = self.game;
         local19: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local20: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(optionsList2Obj, 9, 260, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local19), bbx: 300, bby: 520, tMarcStyle: true, overruleFont: ( local20));
        self.OptionsList2Id = self.AddSubPart( tsubpart2, 300, 520, 260, 160, 0);
      }
      DrawMod.DrawTextColouredMarc( graphics1, "SUPPLY & FUEL STATS", self.game.MarcFont5, 300, 500, Color.White);
      bool flag = false;
      if (self.game.Data.SFTypeObj[typ].ArtRange > 0 | self.game.Data.SFTypeObj[typ].directRange > 0)
        flag = true;
      if (self.comparenr > -1 && self.game.Data.SFTypeObj[self.comparenr].ArtRange > 0 | self.game.Data.SFTypeObj[self.comparenr].directRange > 0)
        flag = true;
      self.OptionsList4Obj = ListClass::new();
      if (flag)
      {
        num1 = 0;
        let mut val2: i32 = Math.Max(self.game.Data.SFTypeObj[typ].ArtRange, self.game.Data.SFTypeObj[typ].directRange);
        if (self.comparenr > -1)
          val2 = Math.Max(Math.Max(self.game.Data.SFTypeObj[self.comparenr].ArtRange, self.game.Data.SFTypeObj[self.comparenr].directRange), val2);
        let mut num10: i32 = Math.Min(9, val2);
        for (let mut index9: i32 = 1; index9 <= num10; index9 += 1)
        {
          let mut num11: i32 = 0;
          str2 = "";
          if (self.game.Data.SFTypeObj[typ].ArtRange >= index9)
            num11 = 100;
          else if (self.game.Data.SFTypeObj[typ].directRange >= index9)
          {
            num11 = self.game.Data.SFTypeObj[typ].directModFirstHex;
            let mut num12: i32 = index9;
            for (let mut index10: i32 = 2; index10 <= num12; index10 += 1)
              num11 =  Math.Round(Math.Floor( (num11 * self.game.Data.SFTypeObj[typ].directModPerHex) / 100.0));
          }
          tvalue11: String = num11.ToString() + "%";
          tvalue2_3: String = "";
          if (self.comparenr > -1)
          {
            let mut num13: i32 = 0;
            if (self.game.Data.SFTypeObj[self.comparenr].ArtRange >= index9)
              num13 = 100;
            else if (self.game.Data.SFTypeObj[self.comparenr].directRange >= index9)
            {
              num13 = self.game.Data.SFTypeObj[self.comparenr].directModFirstHex;
              let mut num14: i32 = index9;
              for (let mut index11: i32 = 2; index11 <= num14; index11 += 1)
                num13 =  Math.Round(Math.Floor( (num13 * self.game.Data.SFTypeObj[self.comparenr].directModPerHex) / 100.0));
            }
            tvalue2_3 = num13.ToString() + "%";
          }
          if (index9 == 9 & val2 > 9)
            self.OptionsList4Obj.add("Range " + index9.ToString() + "+", -1, tvalue11, tvalue2_3);
          else
            self.OptionsList4Obj.add("Range " + index9.ToString(), -1, tvalue11, tvalue2_3);
        }
      }
      if (self.OptionsList4Id > 0)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList4Id)].Refresh(self.OptionsList4Obj, -1);
        self.SubPartFlag[self.SubpartNr(self.OptionsList4Id)] = true;
      }
      else
      {
        ListClass optionsList4Obj = self.OptionsList4Obj;
        let mut game: GameClass = self.game;
         local21: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local22: Font =  font;
        let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsList4Obj, 9, 260, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local21), bbx: 600, bby: 520, tMarcStyle: true, overruleFont: ( local22));
        self.OptionsList4Id = self.AddSubPart( tsubpart3, 600, 520, 260, 160, 0);
      }
      DrawMod.DrawTextColouredMarc( graphics1, "RANGED WEAPONS", self.game.MarcFont5, 600, 500, Color.White);
      StringListClass tListobj1 = new StringListClass(1);
      let mut index12: i32 = -1;
      let mut num15: i32 = -1;
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      tListobj1.AddCol(0, "");
      if ( self.game.Data.RuleVar[492] < 1.0)
        tListobj1.TempColumBmp[0] = Conversions.ToString(self.ReturnSFSpriteNr(typ, regnr, pplnr));
      tListobj1.ColumnName[1] = self.game.Data.SFTypeObj[typ].Ratio <= 1 ? Strings.UCase(self.game.Data.SFTypeObj[typ].Name) + " stats" : Strings.UCase(self.game.Data.SFTypeObj[typ].Name) + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[typ].Ratio)) + "x) stats";
      let mut index13: i32 = 0;
      do
      {
        if (Strings.Len(self.game.Data.SFTypeObj[typ].LogoString[index13]) > 0 && Operators.CompareString(self.game.Data.SFTypeObj[typ].LogoString[index13], "0", false) != 0)
        {
          if ((num15 + 10) % 2 > 0)
          {
            index12 += 1;
            num15 += 1;
            tListobj1.AddRow(tListobj1.Length);
            if ( self.game.Data.RuleVar[947] == 1.0)
            {
              if (Strings.Len(self.game.Data.TempString[1000 + index13]) > 0)
              {
                tListobj1.TempBmp[index12, 0] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index13])];
                tListobj1.TempDesc[index12, 0] = self.game.Data.TempString[1100 + index13];
              }
            }
            else if (Strings.Len(self.game.Data.TempString[1000 + index13]) > 0)
            {
              tListobj1.TempBmp[index12, 0] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index13])];
              tListobj1.TempDesc[index12, 0] = self.game.Data.TempString[1100 + index13];
            }
            tListobj1.Data[index12, 1] = self.game.Data.SFTypeObj[typ].LogoString[index13];
            tListobj1.TempDesc[index12, 1] = self.game.Data.TempString[1100 + index13];
          }
          else
          {
            num15 += 1;
            if ( self.game.Data.RuleVar[947] == 1.0)
            {
              if (Strings.Len(self.game.Data.TempString[1000 + index13]) > 0)
              {
                tListobj1.TempBmp[index12, 2] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index13])];
                tListobj1.TempDesc[index12, 2] = self.game.Data.TempString[1100 + index13];
              }
            }
            else if (Strings.Len(self.game.Data.TempString[1000 + index13]) > 0)
            {
              tListobj1.TempBmp[index12, 2] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index13])];
              tListobj1.TempDesc[index12, 2] = self.game.Data.TempString[1100 + index13];
            }
            tListobj1.Data[index12, 3] = self.game.Data.SFTypeObj[typ].LogoString[index13];
            tListobj1.TempDesc[index12, 3] = self.game.Data.TempString[1100 + index13];
          }
        }
        index13 += 1;
      }
      while (index13 <= 99);
      let mut tlistsize1: i32 = 4;
      let mut num16: i32 = 110;
      if (self.comparenr == -1)
        num16 = 77;
      let mut tsubpart4: SubPartClass =  new MatrixSubPartClass(tListobj1, tlistsize1, 260, -1, -1, self.game, tbackbitmap: ( self.OwnBitmap), bbx: num2, bby: num16, trowheight: 26, tfontsize: 18, tfontoffsety: 2, tnolines: true, tMarcy: true, tMinColValue: 40);
      self.LogoListId = self.AddSubPart( tsubpart4, num2, num16, 260, (tlistsize1 + 3) * 26, 0);
      if (self.comparenr > -1)
      {
        StringListClass tListobj2 = new StringListClass(1);
        let mut index14: i32 = -1;
        let mut num17: i32 = -1;
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        tListobj2.AddCol(0, "");
        if ( self.game.Data.RuleVar[492] < 1.0)
          tListobj2.TempColumBmp[0] = Conversions.ToString(self.ReturnSFSpriteNr(self.comparenr, regnr, pplnr));
        tListobj2.ColumnName[1] = self.game.Data.SFTypeObj[self.comparenr].Ratio <= 1 ? Strings.UCase(self.game.Data.SFTypeObj[self.comparenr].Name) : Strings.UCase(self.game.Data.SFTypeObj[self.comparenr].Name) + " (" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.comparenr].Ratio)) + "x)";
        let mut index15: i32 = 0;
        do
        {
          if (Strings.Len(self.game.Data.SFTypeObj[self.comparenr].LogoString[index15]) > 0 && Operators.CompareString(self.game.Data.SFTypeObj[self.comparenr].LogoString[index15], "0", false) != 0)
          {
            if ((num17 + 10) % 2 > 0)
            {
              index14 += 1;
              num17 += 1;
              tListobj2.AddRow(tListobj2.Length);
              if ( self.game.Data.RuleVar[947] == 1.0)
              {
                if (Strings.Len(self.game.Data.TempString[1000 + index15]) > 0)
                {
                  tListobj2.TempBmp[index14, 0] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index15])];
                  tListobj2.TempDesc[index14, 0] = self.game.Data.TempString[1100 + index15];
                }
              }
              else if (Strings.Len(self.game.Data.TempString[1000 + index15]) > 0)
              {
                tListobj2.TempBmp[index14, 0] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index15])];
                tListobj2.TempDesc[index14, 0] = self.game.Data.TempString[1100 + index15];
              }
              tListobj2.Data[index14, 1] = self.game.Data.SFTypeObj[self.comparenr].LogoString[index15];
              tListobj2.TempDesc[index14, 1] = self.game.Data.TempString[1100 + index15];
            }
            else
            {
              num17 += 1;
              if ( self.game.Data.RuleVar[947] == 1.0)
              {
                if (Strings.Len(self.game.Data.TempString[1000 + index15]) > 0)
                {
                  tListobj2.TempBmp[index14, 2] = self.game.Data.SmallPicNr[Conversions.ToInteger(self.game.Data.TempString[1000 + index15])];
                  tListobj2.TempDesc[index14, 2] = self.game.Data.TempString[1100 + index15];
                }
              }
              else if (Strings.Len(self.game.Data.TempString[1000 + index15]) > 0)
              {
                tListobj2.TempBmp[index14, 2] = self.game.NATO[Conversions.ToInteger(self.game.Data.TempString[1000 + index15])];
                tListobj2.TempDesc[index14, 2] = self.game.Data.TempString[1100 + index15];
              }
              tListobj2.Data[index14, 3] = self.game.Data.SFTypeObj[self.comparenr].LogoString[index15];
              tListobj2.TempDesc[index14, 3] = self.game.Data.TempString[1100 + index15];
            }
          }
          index15 += 1;
        }
        while (index15 <= 99);
        let mut tlistsize2: i32 = 4;
        let mut tsubpart5: SubPartClass =  new MatrixSubPartClass(tListobj2, tlistsize2, 260, -1, -1, self.game, tbackbitmap: ( self.OwnBitmap), bbx: num3, bby: 110, trowheight: 26, tfontsize: 18, tfontoffsety: 2, tnolines: true, tMarcy: true, tMinColValue: 40);
        self.logolist3id = self.AddSubPart( tsubpart5, num3, 110, 260, (tlistsize2 + 3) * 26, 0);
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
          float num18 = self.game.Data.PeopleObj[self.game.Data.SFObj[self.sfnr].People].BattleForMod[self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].People].PeopleGroup];
          if ( num18 > 1.0 |  num18 < 1.0)
          {
            let mut Number: i32 =  Math.Round( ((num18 - 1f) * 100f));
            tvalue12: String = Strings.Trim(Conversion.Str( Number)) + "%";
            if (Number >= 0)
              tvalue12 = "+" + tvalue12;
            self.OptionsList3Obj.add("People Combat Bonus", -1, tvalue12);
          }
          if (reconMinusHide.x == 3)
          {
            self.OptionsList3Obj.add("Qty", -1, Strings.Trim(Conversion.Str( (self.game.Data.SFObj[self.sfnr].Qty * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Ratio))));
            self.OptionsList3Obj.add("Readiness", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Rdn)));
            self.OptionsList3Obj.add("Morale", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Mor)));
            str2 = Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Mor));
            self.OptionsList3Obj.add("Base Morale", -1, ( Math.Round( self.game.Data.PeopleObj[pplnr].BaseMorale[self.game.Data.RegimeObj[regnr].People] * ( self.game.Data.RegimeObj[regnr].BaseMorale / 100.0))).ToString());
            self.OptionsList3Obj.add("Experience", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Xp)));
            self.OptionsList3Obj.add("Current entrenchment", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].CurrentEntrench)));
            self.OptionsList3Obj.add("Entrenchment Cap.", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].EntrenchPower)));
            self.OptionsList3Obj.add("Action Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Ap)));
            self.OptionsList3Obj.add("Engineer Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].EP)));
            self.OptionsList3Obj.add("Recon Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].ReconPts)));
            self.OptionsList3Obj.add("ZOC Points", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].ZOCPts)));
            self.OptionsList3Obj.add("Height cost modifier", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].heightLevelDiff)) + "%");
            self.OptionsList3Obj.add("EP per Turn", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].EP)));
            self.OptionsList3Obj.add("Ratio", -1, "x" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Ratio)));
            self.OptionsList3Obj.add("Individuals", -1, Strings.Trim(Conversion.Str( self.game.Data.SFObj[self.sfnr].Qty)));
            self.OptionsList3Obj.add("Weight/Carry", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Weight)) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].CarryCap)));
            self.OptionsList3Obj.add("Manpower/Carry", -1, Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].manpower)) + "/" + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].manpowerCarry)));
            tvalue13: String = self.game.Data.TempString[self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].MoveType];
            self.OptionsList3Obj.add("Move Type", -1, tvalue13);
            let mut num19: i32 = self.game.Data.transportMovementType[self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].MoveType];
            if (num19 == 0)
              tvalue13 = "Neither";
            if (num19 == 1)
              tvalue13 = "Transporter";
            if (num19 == 2)
              tvalue13 = "Transportable";
            self.OptionsList3Obj.add("Transport Type", -1, tvalue13);
            self.OptionsList3Obj.add("Power Points", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].PowerPts.ToString());
            self.OptionsList3Obj.add("Rear Area 'backbench'", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].BackBench.ToString());
            self.OptionsList3Obj.add("Attacks", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].Attacks.ToString());
            self.OptionsList3Obj.add("Unit Group", -1, self.game.Data.TempString[400 + self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].UnitGroup]);
            self.OptionsList3Obj.add("TargetChance%", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].targettedByRangedChance.ToString());
            self.OptionsList3Obj.add("Hide Points", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].HidePts.ToString());
            self.OptionsList3Obj.add("Favorite Tries", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].FavTargetTries.ToString());
            self.OptionsList3Obj.add("Out of fuel AP multiplier", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].OutOfFuelMove.ToString() + "x");
            self.OptionsList3Obj.add("Rdn loss for 100AP moved", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].ReadinessLoss.ToString());
            if (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].StaffPts > 0)
            {
              self.OptionsList3Obj.add("Staff Points", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].StaffPts.ToString());
              self.OptionsList3Obj.add("Staff Combat Mod", -1, ( Math.Round( (100f * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].StaffCombatMod))).ToString() + "%");
              self.OptionsList3Obj.add("Staff Morale Mod", -1, ( Math.Round( (100f * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].StaffMoraleMod))).ToString() + "%");
            }
            self.OptionsList3Obj.add("Attack startup penalty", -1, (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].FirstRoundPenaltyMod * 100f).ToString() + "%");
            self.OptionsList3Obj.add("Max attacked", -1, self.game.Data.SFTypeObj[self.game.Data.SFObj[self.sfnr].Type].MaxAttacked.ToString() + "%");
          }
          self.OptionsList3Obj.Sort();
          if (self.OptionsList3Id > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsList3Id)].Refresh(self.OptionsList3Obj, -1);
            self.SubPartFlag[self.SubpartNr(self.OptionsList3Id)] = true;
          }
          else
          {
            ListClass optionsList3Obj = self.OptionsList3Obj;
            let mut game: GameClass = self.game;
             local23: Bitmap =  self.OwnBitmap;
            font: Font =  null;
             local24: Font =  font;
            let mut tsubpart6: SubPartClass =  new ListSubPartClass(optionsList3Obj, 22, 260, -1, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 100, tdotopandbottom: false, tbackbitmap: ( local23), bbx: 20, bby: 308, tMarcStyle: true, overruleFont: ( local24));
            self.OptionsList3Id = self.AddSubPart( tsubpart6, 20, 308, 260, 460, 0);
          }
          DrawMod.DrawTextColouredMarc( graphics1, "SELECTED TROOPS STATS", self.game.MarcFont5, 90, 291, Color.White);
          rectangle1 = Rectangle::new(20, 335, 290, 144);
          let mut trect: &Rectangle = &rectangle1
          self.AddMouse( trect, "", "The troops in the slot you clicked\r\nhave their own detailed stats.");
        }
      }
      let mut tsubpart7: SubPartClass =  new TextButtonPartClass("OK", 150, "Click to return to main screen.",  self.OwnBitmap,  Math.Round( self.OwnBitmap.Width / 2.0 + 15.0), 703, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but1id = self.AddSubPart( tsubpart7,  Math.Round( self.OwnBitmap.Width / 2.0 + 15.0), 703, 150, 40, 1);
      tsubpart7 =  new TextButtonPartClass("COMPARE", 150, "Click to select another trooptype to compare stats with.",  self.OwnBitmap,  Math.Round( self.OwnBitmap.Width / 2.0 - 175.0), 703, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.but7id = self.AddSubPart( tsubpart7,  Math.Round( self.OwnBitmap.Width / 2.0 - 175.0), 703, 150, 40, 1);
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
