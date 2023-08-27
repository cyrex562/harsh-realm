// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATFlexMessageWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ATFlexMessageWindowClass : WindowClass
  {
     int okid;
     int cancelid;
     int oktextid;
     int Pic1Id;
     int TAid;
     int His;
     int Card;
     int Unr;
     int[] Answer;

    pub ATFlexMessageWindowClass(ref tGame: GameClass)
      : base(ref tGame, 800, 200, BackSprite: tGame.BACKGROUND2MARC, tBackSpriteScaled: true)
    {
      this.Answer = new int[10];
      this.View();
    }

    pub fn View()
    {
      this.ClearMouse();
      this.NewBackGroundAndClearAll(800, 200, DrawMod.TGame.BACKGROUND2MARC);
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      SizeF sizeF1 = SizeF::new();
      str1: String = this.game.EditObj.QuestionText;
      sizeF1 = objgraphics.MeasureString(str1, this.game.VicFont3);
      let mut num1: i32 =  -1;
      while (Strings.Len(str1) > 0)
      {
        num1 += 1;
        let mut Length: i32 =  Strings.InStr(str1, ".");
        let mut num2: i32 =  0;
        if (Length > 0)
          num2 = Strings.InStr(Length + 1, str1, ".");
        str2: String;
        if (Length > 0 & num2 > 0)
        {
          str2 = Strings.Left(str1, Length);
          str1 = Strings.Mid(str1, Length + 1);
        }
        else
        {
          str2 = str1;
          str1 = "";
        }
        SizeF sizeF2 = objgraphics.MeasureString(str2, this.game.VicFont3);
        DrawMod.DrawTextVic2(ref objgraphics, str2, this.game.VicFont3, (int) Math.Round(400.0 -  sizeF2.Width / 2.0), 40 + num1 * 20, this.game.VicColor2, this.game.VicColor2Shade);
      }
      int[] answer = this.Answer;
      let mut tsubpart: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[1], 100, tBackbitmap: (ref this.OwnBitmap), bbx: 350, bby: 150);
      let mut num3: i32 =  this.AddSubPart(ref tsubpart, 350, 150, 100, 36, 1);
      answer[1] = num3;
      Rectangle trect = Rectangle::new(350, 150, 100, 35);
      this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[1]);
    }

    pub HandleKeyPress: WindowReturnClass(int nr, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27)
        {
          this.game.EditObj.AnswerChosen = 2;
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 32)
        {
          this.game.EditObj.AnswerChosen = 1;
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut answerCount: i32 =  this.game.EditObj.AnswerCount;
            for (let mut index2: i32 =  1; index2 <= answerCount; index2 += 1)
            {
              if (this.Answer[index2] == this.SubPartID[index1])
              {
                windowReturnClass.AddCommand(6, 0);
                this.game.EditObj.AnswerChosen = index2;
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
  }
}
