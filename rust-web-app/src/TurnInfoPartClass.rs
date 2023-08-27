// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TurnInfoPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TurnInfoPartClass : SubPartClass
  {
     object OwnBitmapNr;
     game: GameClass;

    pub TurnInfoPartClass(tgame: GameClass)
      : base(200, 50)
    {
      self.game = tgame;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      let mut turn: i32 = self.game.Data.Turn;
      let mut round: i32 = self.game.Data.Round;
      red2: i32;
      green2: i32;
      blue2: i32;
      if (round > 0)
      {
        let mut red: i32 = self.game.Data.RegimeObj[turn].Red;
        let mut green: i32 = self.game.Data.RegimeObj[turn].Green;
        let mut blue: i32 = self.game.Data.RegimeObj[turn].Blue;
        red2 = self.game.Data.RegimeObj[turn].Red2;
        green2 = self.game.Data.RegimeObj[turn].Green2;
        blue2 = self.game.Data.RegimeObj[turn].Blue2;
        c1: Color = Color.FromArgb( byte.MaxValue, red, green, blue);
        c2: Color = Color.FromArgb(150, red, green, blue);
        DrawMod.DrawBlockGradient( Expression, 0, 0, 200, 50, c1, c2);
      }
      else
      {
        c1: Color = Color.FromArgb( byte.MaxValue, 180, 180, 180);
        c2: Color = Color.FromArgb(150, 90, 90, 90);
        DrawMod.DrawBlockGradient( Expression, 0, 0, 200, 50, c1, c2);
      }
      if (round > 0)
      {
        c: Color = Color.FromArgb( byte.MaxValue, red2, green2, blue2);
        str: String;
        if (self.game.Data.AlternateRound == -1)
        {
          str = "Round " + Conversion.Str( self.game.Data.Round);
          if (self.game.Data.AlternateRound2 > 0)
          {
            DateTime dateTime = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1)).AddHours( self.game.Data.StartData.Hour);
            TimeSpan timeSpan = new TimeSpan(0, (self.game.Data.Round - 1) * self.game.Data.AlternateRound2, 0, 0);
            dateTime = dateTime.Add(timeSpan);
            str = Strings.Trim(Conversion.Str( dateTime.Day)) + "-" + Strings.Trim(Conversion.Str( dateTime.Month)) + "-" + Strings.Trim(Conversion.Str( dateTime.Year)) + " " + Strings.Trim(Conversion.Str( dateTime.Hour)) + ":00";
          }
        }
        else
        {
          DateTime dateTime1 = DateTime::new().AddYears(self.game.Data.StartData.Year - 1).AddMonths(self.game.Data.StartData.Month - 1).AddDays( (self.game.Data.StartData.Day - 1));
          DateTime dateTime2;
          if (self.game.Data.AlternateRound == 31)
          {
            dateTime2 = dateTime1.AddMonths((self.game.Data.Round - 1) * 1);
          }
          else
          {
            TimeSpan timeSpan = new TimeSpan((self.game.Data.Round - 1) * self.game.Data.AlternateRound, 0, 0, 0);
            dateTime2 = dateTime1.Add(timeSpan);
          }
          str = Strings.Trim(Conversion.Str( dateTime2.Day)) + "-" + Strings.Trim(Conversion.Str( dateTime2.Month)) + "-" + Strings.Trim(Conversion.Str( dateTime2.Year));
        }
        sizeF1 = Expression.MeasureString(str, Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel));
        DrawMod.DrawTextColoured( Expression, str, Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 10, 5, c);
        name: String = self.game.Data.RegimeObj[self.game.Data.Turn].Name;
        sizeF1 = Expression.MeasureString(name, Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel));
        DrawMod.DrawTextColoured( Expression, name, Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 10, 25, c);
        tstring: String = "PolPts: " + Strings.Trim(Conversion.Str( self.game.Data.RegimeObj[self.game.Data.Turn].ResPts));
        DrawMod.DrawTextColoured( Expression, tstring, Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 120, 5, c);
      }
      else
      {
        c: Color = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        str: String = "Editor Mode";
        SizeF sizeF2 = Expression.MeasureString(str, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel));
        DrawMod.DrawTextColoured( Expression, str, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel),  Math.Round(100.0 -  sizeF2.Width / 2.0), 5, c);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
       let mut local1: &Graphics = &Expression;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(self.OwnBitmapNr));
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }
  }
}
