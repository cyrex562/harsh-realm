// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.FlexMessageWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class FlexMessageWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     TAid: i32;
     His: i32;
     Card: i32;
     Unr: i32;
     int[] Answer;

    pub FlexMessageWindowClass(ref tGame: GameClass)
      : base(ref tGame, 600, 200, 8)
    {
      this.Answer = new int[10];
      this.View();
    }

    pub fn View()
    {
      this.ClearMouse();
      this.NewBackGroundAndClearAll(600, 200, -1);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref graphics, 0, 0, 600, 200);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      SizeF sizeF1 = SizeF::new();
      str1: String = this.game.EditObj.QuestionText;
      sizeF1 = graphics.MeasureString(str1, this.game.MarcFont4);
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
        SizeF sizeF2 = graphics.MeasureString(str2, this.game.MarcFont4);
        DrawMod.DrawTextColouredMarc(ref graphics, str2, this.game.MarcFont4,  Math.Round(300.0 -  sizeF2.Width / 2.0), 40 + num1 * 20, Color.White);
      }
      if (this.game.EditObj.AnswerCount == 1)
      {
        int[] answer = this.Answer;
        let mut tsubpart: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[1], 100, tBackbitmap: (ref this.OwnBitmap), bbx: 250, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num3: i32 =  this.AddSubPart(ref tsubpart, 250, 100, 100, 36, 1);
        answer[1] = num3;
        Rectangle trect = Rectangle::new(250, 100, 100, 35);
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[1]);
      }
      else if (this.game.EditObj.AnswerCount == 2)
      {
        int[] answer1 = this.Answer;
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[1], 100, tBackbitmap: (ref this.OwnBitmap), bbx: 175, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num4: i32 =  this.AddSubPart(ref tsubpart1, 175, 100, 100, 36, 1);
        answer1[1] = num4;
        Rectangle rectangle = Rectangle::new(175, 100, 100, 35);
        let mut trect: &Rectangle = &rectangle
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[1]);
        int[] answer2 = this.Answer;
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[2], 100, tBackbitmap: (ref this.OwnBitmap), bbx: 325, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num5: i32 =  this.AddSubPart(ref tsubpart2, 325, 100, 100, 36, 1);
        answer2[2] = num5;
        rectangle = Rectangle::new(325, 100, 100, 35);
        trect = rectangle;
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[2]);
      }
      else
      {
        if (this.game.EditObj.AnswerCount != 3)
          return;
        int[] answer3 = this.Answer;
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[1], 166, tBackbitmap: (ref this.OwnBitmap), bbx: 25, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num6: i32 =  this.AddSubPart(ref tsubpart3, 25, 100, 166, 36, 1);
        answer3[1] = num6;
        Rectangle rectangle = Rectangle::new(25, 100, 166, 36);
        let mut trect: &Rectangle = &rectangle
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[1]);
        int[] answer4 = this.Answer;
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[2], 166, tBackbitmap: (ref this.OwnBitmap), bbx: 216, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num7: i32 =  this.AddSubPart(ref tsubpart4, 216, 100, 166, 36, 1);
        answer4[2] = num7;
        rectangle = Rectangle::new(216, 100, 166, 36);
        trect = rectangle;
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[2]);
        int[] answer5 = this.Answer;
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass(this.game.EditObj.AnswerText[3], 166, tBackbitmap: (ref this.OwnBitmap), bbx: 407, bby: 100, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        let mut num8: i32 =  this.AddSubPart(ref tsubpart5, 407, 100, 166, 36, 1);
        answer5[3] = num8;
        rectangle = Rectangle::new(407, 100, 166, 36);
        trect = rectangle;
        this.AddMouse(ref trect, "", this.game.EditObj.AnswerTextMouseOver[3]);
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
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

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
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
