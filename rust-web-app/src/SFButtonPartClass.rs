// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SFButtonPartClass : SubPartClass
  {
    pub OwnBitmapNr: i32;
     colorized: i32;
     bool overrule;
     resizex: i32;
     resizey: i32;
     typ: i32;
     regnr: i32;
     extra: i32;

    pub SFButtonPartClass(ttyp: i32, tregnr: i32, tResizeX: i32, tresizeY: i32)
      : base(tResizeX, tresizeY)
    {
      self.overrule = false;
      self.resizex = tResizeX;
      self.typ = ttyp;
      self.extra = tregnr;
      self.regnr = DrawMod.TGame.Data.Turn;
      self.resizey = tresizeY;
    }

    pub SFButtonPartClass(tbmpnr: Bitmap, tDescript: String = "")
      : base(tbmpnr.Width, tbmpnr.Height)
    {
      self.OwnBitmap = (Bitmap) tbmpnr.Clone();
      self.overrule = true;
      self.Descript = tDescript;
    }

    pub Paint: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      let mut picSpriteId: i32 = DrawMod.TGame.Data.SFTypeObj[self.typ].PicSpriteID;
      let mut sidewaysSpriteId: i32 = DrawMod.TGame.Data.SFTypeObj[self.typ].SidewaysSpriteID;
      let mut extraCounter: i32 = DrawMod.TGame.Data.SFTypeObj[self.typ].ExtraCounter;
      for (let mut index: i32 = 0; index <= extraCounter; index += 1)
      {
        if (DrawMod.TGame.Data.SFTypeObj[self.typ].ExtraCode[index] == self.extra)
        {
          picSpriteId = DrawMod.TGame.Data.SFTypeObj[self.typ].ExtraPicSpriteID[index];
          sidewaysSpriteId = DrawMod.TGame.Data.SFTypeObj[self.typ].ExtraSidewaysSpriteID[index];
        }
      }
      let mut x1: i32 = 0;
      let mut y1: i32 = 0;
      let mut width1: i32 = self.OwnBitmap.Width;
      let mut height: i32 = self.OwnBitmap.Height;
      index1: i32;
      index2: i32;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if ( DrawMod.TGame.Data.RuleVar[869] >= 1.0)
      {
        index1 =  Math.Round( DrawMod.TGame.Data.RuleVar[873]);
        index2 = 0;
        if ( DrawMod.TGame.Data.RuleVar[848] > 0.0 & DrawMod.TGame.Data.SFTypeObj[self.typ].Theater == 2)
        {
          index1 =  Math.Round( DrawMod.TGame.Data.RuleVar[848]);
          index2 = 0;
        }
        if ( DrawMod.TGame.Data.RuleVar[872] > 0.0 & DrawMod.TGame.Data.SFTypeObj[self.typ].Theater == 1)
        {
          index1 =  Math.Round( DrawMod.TGame.Data.RuleVar[872]);
          index2 = 0;
        }
        if ( DrawMod.TGame.Data.RuleVar[869] == 3.0)
        {
          let mut nr: i32 = DrawMod.TGame.Data.LandscapeTypeObj[index1].BasicPicID[index2];
           let mut local1: &Graphics = &Expression;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local2: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(x1, y1, width1, height);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
        }
        else
        {
          if ( DrawMod.TGame.Data.RuleVar[869] == 1.0)
          {
            let mut nr: i32 = DrawMod.TGame.Data.LandscapeTypeObj[index1].SidewaysSPriteID1[index2];
             let mut local3: &Graphics = &Expression;
            bitmap: Bitmap = BitmapStore.GetBitmap(nr);
             let mut local4: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x1, y1, width1, height);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
          }
          let mut nr1: i32 = DrawMod.TGame.Data.LandscapeTypeObj[index1].SidewaysSPriteID2[index2];
           let mut local5: &Graphics = &Expression;
          bitmap1: Bitmap = BitmapStore.GetBitmap(nr1);
           let mut local6: &Bitmap = &bitmap1;
          rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr1));
          let mut srcrect1: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x1, y1, width1, height);
          let mut destrect1: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
        }
      }
      let mut index3: i32 = self.regnr;
      if (index3 == -1)
      {
        if (DrawMod.TGame.Data.RegimeCounter == -1)
          return self.OwnBitmap;
        index3 = 0;
      }
      let mut red: i32 = DrawMod.TGame.Data.RegimeObj[index3].Red;
      let mut green: i32 = DrawMod.TGame.Data.RegimeObj[index3].Green;
      let mut blue: i32 = DrawMod.TGame.Data.RegimeObj[index3].Blue;
      switch (DrawMod.TGame.Data.SFTypeObj[self.typ].BaseColor)
      {
        case 0:
           let mut local7: &Graphics = &Expression;
          bitmap2: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local8: &Bitmap = &bitmap2;
          let mut x2: i32 = x1;
          let mut y2: i32 = y1;
          let mut w1: i32 = width1;
          let mut h1: i32 = height;
          DrawMod.DrawScaled( local7,  local8, x2, y2, w1, h1);
          break;
        case 1:
           let mut local9: &Graphics = &Expression;
          bitmap3: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local10: &Bitmap = &bitmap3;
          let mut x3: i32 = x1;
          let mut y3: i32 = y1;
          let mut w2: i32 = width1;
          let mut h2: i32 = height;
          let mut width2: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh1: i32 = BitmapStore.Getheight(picSpriteId);
          double r1 =  ( red / 256f);
          double g1 =  ( green / 256f);
          double b1 =  ( blue / 256f);
          DrawMod.DrawScaledColorized2( local9,  local10, x3, y3, w2, h2, width2, origh1,  r1,  g1,  b1, 1f);
          break;
        case 2:
          let mut red2: i32 = DrawMod.TGame.Data.RegimeObj[index3].Red2;
          let mut green2: i32 = DrawMod.TGame.Data.RegimeObj[index3].Green2;
          let mut blue2: i32 = DrawMod.TGame.Data.RegimeObj[index3].Blue2;
           let mut local11: &Graphics = &Expression;
          bitmap4: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local12: &Bitmap = &bitmap4;
          let mut x4: i32 = x1;
          let mut y4: i32 = y1;
          let mut w3: i32 = width1;
          let mut h3: i32 = height;
          let mut width3: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh2: i32 = BitmapStore.Getheight(picSpriteId);
          double r2 =  ( red2 / 256f);
          double g2 =  ( green2 / 256f);
          double b2 =  ( blue2 / 256f);
          DrawMod.DrawScaledColorized2( local11,  local12, x4, y4, w3, h3, width3, origh2,  r2,  g2,  b2, 1f);
          break;
        case 3:
          let mut red3: i32 = DrawMod.TGame.Data.RegimeObj[index3].Red3;
          let mut green3: i32 = DrawMod.TGame.Data.RegimeObj[index3].Green3;
          let mut blue3: i32 = DrawMod.TGame.Data.RegimeObj[index3].Blue3;
           let mut local13: &Graphics = &Expression;
          bitmap5: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local14: &Bitmap = &bitmap5;
          let mut x5: i32 = x1;
          let mut y5: i32 = y1;
          let mut w4: i32 = width1;
          let mut h4: i32 = height;
          let mut width4: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh3: i32 = BitmapStore.Getheight(picSpriteId);
          double r3 =  ( red3 / 256f);
          double g3 =  ( green3 / 256f);
          double b3 =  ( blue3 / 256f);
          DrawMod.DrawScaledColorized2( local13,  local14, x5, y5, w4, h4, width4, origh3,  r3,  g3,  b3, 1f);
          break;
        case 4:
          let mut red4: i32 = DrawMod.TGame.Data.RegimeObj[index3].Red4;
          let mut green4: i32 = DrawMod.TGame.Data.RegimeObj[index3].Green4;
          let mut blue4: i32 = DrawMod.TGame.Data.RegimeObj[index3].Blue4;
           let mut local15: &Graphics = &Expression;
          bitmap6: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local16: &Bitmap = &bitmap6;
          let mut x6: i32 = x1;
          let mut y6: i32 = y1;
          let mut w5: i32 = width1;
          let mut h5: i32 = height;
          let mut width5: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh4: i32 = BitmapStore.Getheight(picSpriteId);
          double r4 =  ( red4 / 256f);
          double g4 =  ( green4 / 256f);
          double b4 =  ( blue4 / 256f);
          DrawMod.DrawScaledColorized2( local15,  local16, x6, y6, w5, h5, width5, origh4,  r4,  g4,  b4, 1f);
          break;
        case 5:
           let mut local17: &Graphics = &Expression;
          bitmap7: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local18: &Bitmap = &bitmap7;
          let mut x7: i32 = x1;
          let mut y7: i32 = y1;
          let mut w6: i32 = width1;
          let mut h6: i32 = height;
          let mut width6: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh5: i32 = BitmapStore.Getheight(picSpriteId);
          double r5 =  ( (red + 392) / 1024f);
          double g5 =  ( (green + 392) / 1024f);
          double b5 =  ( (blue + 392) / 1024f);
          DrawMod.DrawScaledColorized2( local17,  local18, x7, y7, w6, h6, width6, origh5,  r5,  g5,  b5, 1f);
          break;
        case 6:
           let mut local19: &Graphics = &Expression;
          bitmap8: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local20: &Bitmap = &bitmap8;
          let mut x8: i32 = x1;
          let mut y8: i32 = y1;
          let mut w7: i32 = width1;
          let mut h7: i32 = height;
          let mut width7: i32 = BitmapStore.GetWidth(picSpriteId);
          let mut origh6: i32 = BitmapStore.Getheight(picSpriteId);
          double r6 =  ( (red + 80) / 512f);
          double g6 =  ( (green + 200) / 512f);
          double b6 =  ( (blue + 80) / 512f);
          DrawMod.DrawScaledColorized2( local19,  local20, x8, y8, w7, h7, width7, origh6,  r6,  g6,  b6, 1f);
          break;
      }
      if ( DrawMod.TGame.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(sidewaysSpriteId)))
      {
         let mut local21: &Graphics = &Expression;
        bitmap9: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
         let mut local22: &Bitmap = &bitmap9;
        let mut x9: i32 = x1;
        let mut y9: i32 = y1;
        let mut w8: i32 = width1;
        let mut h8: i32 = height;
        DrawMod.DrawScaled( local21,  local22, x9, y9, w8, h8);
      }
      if ( DrawMod.TGame.Data.RuleVar[869] >= 1.0 &  DrawMod.TGame.Data.RuleVar[869] < 3.0)
      {
        let mut nr: i32 = DrawMod.TGame.Data.LandscapeTypeObj[index1].SidewaysSPriteID3[index2];
         let mut local23: &Graphics = &Expression;
        bitmap10: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local24: &Bitmap = &bitmap10;
        rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, y1, width1, height);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local23,  local24, srcrect, destrect);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap() => self.Paint();
  }
}
